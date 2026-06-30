import numpy as np
import pytest

from rlbot_flatbuffers import *


class TestVector:
    def test_custom_subclass(self):
        class MyVector(Vector3):
            def __add__(self, other):
                return MyVector(self.x + other.x, self.y + other.y, self.z + other.z)

        vec1 = MyVector(1, 2, 3)
        vec2 = Vector3(4, 5, 6)
        vec3 = vec1 + vec2
        assert vec3.x == 5
        assert vec3.y == 7
        assert vec3.z == 9


class TestController:
    def test_controller_state(self):
        controller = ControllerState()
        controller.throttle = 1
        controller.steer = 0.5
        controller.pitch = np.array([0.1], dtype=np.float32)[0]
        controller.pack()


class TestEnums:
    def test_air_state_match(self):
        air_state = AirState.Dodging
        match air_state:
            case AirState.Dodging:
                pass
            case _:
                pytest.fail("Should have matched Dodging")

    def test_air_state_invalid_value(self):
        with pytest.raises(ValueError, match="Unknown value of 8"):
            AirState(8)


class TestReprRoundtrip:
    """eval(repr(x)) produces the same type with the same key fields."""

    def test_player_info(self):
        player_info = PlayerInfo(name="HELLO", accolades=["MVP", "Hat Trick"])
        restored = eval(repr(player_info))
        assert isinstance(restored, PlayerInfo)
        assert restored.name == "HELLO"
        assert list(restored.accolades) == ["MVP", "Hat Trick"]

    def test_connection_settings(self):
        cs = ConnectionSettings("rlbot/abot", True, close_between_matches=True)
        restored = eval(repr(cs))
        assert isinstance(restored, ConnectionSettings)
        assert restored.agent_id == "rlbot/abot"
        assert restored.wants_ball_predictions is True
        assert restored.close_between_matches is True

    def test_desired_game_state(self):
        dgs = DesiredGameState(match_info=DesiredMatchInfo(game_speed=2))
        restored = eval(repr(dgs))
        assert isinstance(restored, DesiredGameState)

    def test_render_message(self):
        rm = RenderMessage()
        restored = eval(repr(rm))
        assert isinstance(restored, RenderMessage)

    def test_render_message_with_line3d(self):
        rm = RenderMessage(
            Line3D(
                RenderAnchor(),
                RenderAnchor(relative=CarAnchor(0, Vector3(1, 1, 1))),
                Color(255),
            )
        )
        restored = eval(repr(rm))
        assert isinstance(restored, RenderMessage)

    def test_match_comm(self):
        comm = MatchComm(3, 1, False, "Ready!", b"Hello, world!")
        restored = eval(repr(comm))
        assert isinstance(restored, MatchComm)


class TestRenderMessageLine3D:
    def test_color_access(self):
        rm = RenderMessage(
            Line3D(
                RenderAnchor(),
                RenderAnchor(relative=CarAnchor(0, Vector3(1, 1, 1))),
                Color(255),
            )
        )
        assert isinstance(rm.variety, Line3D)
        assert rm.variety.color.r == 255
        assert rm.variety.color.a == 255

    def test_color_mutation(self):
        rm = RenderMessage(
            Line3D(
                RenderAnchor(),
                RenderAnchor(relative=CarAnchor(0, Vector3(1, 1, 1))),
                Color(255),
            )
        )
        rm.variety.color.a = 150
        assert rm.variety.color.a == 150


class TestDesiredGameState:
    def test_access_and_mutation(self):
        dgs = DesiredGameState(match_info=DesiredMatchInfo(game_speed=2))

        assert dgs.match_info is not None
        dgs.match_info.world_gravity_z = -650

        assert dgs.match_info.game_speed is not None
        dgs.match_info.game_speed += 1
        assert dgs.match_info.game_speed == 3

        dgs.console_commands = [ConsoleCommand("dump_items")]
        dgs.ball_states = [DesiredBallState()]


class TestErrors:
    def test_invalid_flatbuffer(self):
        comm = MatchComm(3, 1, False, "Ready!", b"Hello, world!")
        invalid_data = comm.pack()
        with pytest.raises(InvalidFlatbuffer):
            RenderMessage.unpack(invalid_data)


class TestMatchConfiguration:
    def test_large_match_config(self):
        match_settings = MatchConfiguration(
            launcher_arg="test",
            game_map_upk="map",
            player_configurations=[
                PlayerConfiguration(
                    variety=CustomBot(
                        name=f"bot{i}",
                        root_dir="/tmp",
                        run_command="echo",
                        agent_id=f"dev/test{i}",
                    ),
                )
                for i in range(128)
            ],
            script_configurations=[
                ScriptConfiguration(
                    name=f"script{i}",
                    root_dir="/tmp",
                    run_command="echo",
                    agent_id=f"dev/test{i}",
                )
                for i in range(8)
            ],
            mutators=MutatorSettings(),
        )
        data = match_settings.pack()
        assert len(data) > 0


class TestPolyLine3D:
    def test_many_points(self):
        rm = RenderMessage(
            PolyLine3D(
                [Vector3() for _ in range(2048)],
                Color(a=255),
            ),
        )
        match rm.variety:
            case PolyLine3D(points, clr):
                assert len(points) == 2048
                assert clr.a == 255
            case _:
                pytest.fail("Expected PolyLine3D")

    def test_pack(self):
        rm = RenderMessage(
            PolyLine3D(
                [Vector3() for _ in range(2048)],
                Color(a=255),
            ),
        )
        data = rm.pack()
        assert len(data) > 0


class TestBallPrediction:
    def test_predictions(self):
        ballPred = BallPrediction([PredictionSlice(1) for _ in range(6 * 120)])
        for i, slice_ in enumerate(ballPred.slices):
            assert slice_.game_seconds - (i / 120 + 1) < 1e-6

    def test_empty(self):
        empty = BallPrediction()
        empty.pack()
        assert len(empty.slices) == 0

    def test_full(self):
        full = BallPrediction([PredictionSlice(t / 120) for t in range(6 * 120)])
        full.pack()
        assert len(full.slices) == 6 * 120

    def test_half(self):
        half = BallPrediction([PredictionSlice(t / 120 + 1) for t in range(3 * 120)])
        half.pack()
        assert len(half.slices) == 3 * 120
