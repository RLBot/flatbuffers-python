import pytest

from rlbot_flatbuffers import *


class TestHumanEmptyTable:
    """Empty tables have special codegen paths."""

    def test_create(self):
        h = Human()
        assert isinstance(h, Human)

    def test_repr(self):
        assert repr(Human()) == "Human()"

    def test_roundtrip(self):
        h = Human()
        h2 = Human.unpack(h.pack())
        assert isinstance(h2, Human)


class TestStructColor:
    """Structs (not tables) have a different codegen path."""

    def test_create_with_args(self):
        c = Color(r=10, g=20, b=30, a=255)
        assert c.r == 10
        assert c.g == 20
        assert c.b == 30
        assert c.a == 255

    def test_defaults(self):
        c = Color()
        assert c.r == 0
        assert c.g == 0
        assert c.b == 0
        assert c.a == 255  # hardcoded override in codegen

    def test_mutation(self):
        c = Color()
        c.r = 100
        c.g = 150
        c.b = 200
        c.a = 255
        assert c.r == 100
        assert c.g == 150
        assert c.b == 200
        assert c.a == 255

    def test_repr(self):
        c = Color(r=255, g=128, b=64, a=255)
        r = repr(c)
        assert "r=255" in r
        assert "g=128" in r
        assert "b=64" in r
        assert "a=255" in r


class TestFloatFields:
    """Float fields use a special Py<PyFloat> type with custom setter."""

    def test_rotator(self):
        r = Rotator(pitch=1.5, yaw=2.5, roll=3.5)
        assert r.pitch == 1.5
        assert r.yaw == 2.5
        assert r.roll == 3.5

    def test_rotator_defaults(self):
        r = Rotator()
        assert r.pitch == 0.0
        assert r.yaw == 0.0
        assert r.roll == 0.0

    def test_rotator_mutation(self):
        r = Rotator()
        r.pitch = 1.0
        assert r.pitch == 1.0

    def test_repr(self):
        r = Rotator(pitch=1.5, yaw=2.5, roll=3.5)
        s = repr(r)
        assert isinstance(s, str) and len(s) > 0


class TestOptionalFields:
    """Tables with optional fields of various types."""

    def test_optional_table(self):
        """CustomBot.loadout is an optional table."""
        bot = CustomBot(name="n", root_dir="r", run_command="c", agent_id="a")
        assert bot.loadout is None

        bot.loadout = PlayerLoadout(car_id=5)
        assert bot.loadout is not None
        assert bot.loadout.car_id == 5

    def test_optional_table_roundtrip(self):
        bot = CustomBot(name="n", root_dir="r", run_command="c", agent_id="a")
        bot2 = CustomBot.unpack(bot.pack())
        assert bot2.loadout is None

        bot.loadout = PlayerLoadout(car_id=5)
        bot3 = CustomBot.unpack(bot.pack())
        assert bot3.loadout is not None
        assert bot3.loadout.car_id == 5

    def test_optional_float(self):
        """DesiredMatchInfo has optional Float fields."""
        info = DesiredMatchInfo()
        assert info.world_gravity_z is None
        assert info.game_speed is None

        info.world_gravity_z = -650.0
        assert info.world_gravity_z == -650.0

    def test_optional_float_roundtrip(self):
        info = DesiredMatchInfo(world_gravity_z=-650.0, game_speed=2.0)
        info2 = DesiredMatchInfo.unpack(info.pack())
        assert info2.world_gravity_z == -650.0
        assert info2.game_speed == 2.0

    def test_optional_enum(self):
        """PsyonixBot.bot_skill is an optional enum."""
        bot = PsyonixBot(name="bot")
        assert bot.bot_skill == PsyonixSkill.Beginner  # default enum value

    def test_optional_table_in_nested(self):
        """DesiredGameState.match_info is an optional table."""
        dgs = DesiredGameState()
        assert dgs.match_info is None

        dgs.match_info = DesiredMatchInfo(game_speed=2.0)
        assert dgs.match_info is not None
        assert dgs.match_info.game_speed == 2.0

    def test_optional_table_nested_roundtrip(self):
        dgs = DesiredGameState(match_info=DesiredMatchInfo(game_speed=2.0))
        dgs2 = DesiredGameState.unpack(dgs.pack())
        assert dgs2.match_info is not None
        assert dgs2.match_info.game_speed == 2.0


class TestVectorStrings:
    """Vectors of strings (PlayerInfo.accolades)."""

    def test_create(self):
        info = PlayerInfo(name="test", accolades=["MVP", "Hat Trick"])
        assert list(info.accolades) == ["MVP", "Hat Trick"]

    def test_default_empty(self):
        info = PlayerInfo(name="test")
        assert len(info.accolades) == 0

    def test_roundtrip(self):
        info = PlayerInfo(name="test", accolades=["MVP", "Hat Trick", "Winner"])
        info2 = PlayerInfo.unpack(info.pack())
        assert list(info2.accolades) == ["MVP", "Hat Trick", "Winner"]

    def test_repr(self):
        info = PlayerInfo(name="test", accolades=["A", "B"])
        r = repr(info)
        assert "A" in r
        assert "B" in r


class TestVectorU8:
    """Vectors of u8 bytes (MatchComm.content)."""

    def test_create(self):
        comm = MatchComm(3, 1, False, "", b"hello")
        assert bytes(comm.content) == b"hello"

    def test_default_empty(self):
        comm = MatchComm(3, 1, False, "")
        assert bytes(comm.content) == b""

    def test_roundtrip(self):
        data = bytes(range(256))
        comm = MatchComm(3, 1, False, "ready", data)
        comm2 = MatchComm.unpack(comm.pack())
        assert bytes(comm2.content) == data

    def test_repr(self):
        comm = MatchComm(3, 1, False, "ready", b"\x00\x01\x02")
        r = repr(comm)
        assert "bytes([" in r


class TestVectorTables:
    """Vectors of tables (PlayerConfiguration in MatchConfiguration)."""

    def test_create(self):
        configs = [
            PlayerConfiguration(
                variety=CustomBot(
                    name="b1", root_dir="/tmp", run_command="c", agent_id="a"
                ),
            )
        ]
        mc = MatchConfiguration(
            launcher_arg="",
            game_map_upk="map",
            player_configurations=configs,
            script_configurations=[],
        )
        assert len(mc.player_configurations) == 1

    def test_empty(self):
        mc = MatchConfiguration(
            launcher_arg="",
            game_map_upk="map",
            player_configurations=[],
            script_configurations=[],
        )
        assert len(mc.player_configurations) == 0

    def test_roundtrip(self):
        configs = [
            PlayerConfiguration(
                variety=CustomBot(
                    name="b1", root_dir="/tmp", run_command="c", agent_id="a"
                ),
                team=1,
            ),
            PlayerConfiguration(
                variety=PsyonixBot(name="b2", bot_skill=PsyonixSkill.AllStar),
                team=0,
            ),
        ]
        mc = MatchConfiguration(
            launcher_arg="test",
            game_map_upk="map",
            player_configurations=configs,
            script_configurations=[],
        )
        mc2 = MatchConfiguration.unpack(mc.pack())
        assert len(mc2.player_configurations) == 2


class TestUnionTypes:
    """Union types (PlayerClass, RenderType, CollisionShape, RelativeAnchor)."""

    def test_player_class_custom_bot(self):
        pc = PlayerConfiguration(
            variety=CustomBot(name="b", root_dir="/tmp", run_command="c", agent_id="a"),
        )
        assert isinstance(pc.variety, CustomBot)

    def test_player_class_psyonix_bot(self):
        pc = PlayerConfiguration(variety=PsyonixBot(name="b"))
        assert isinstance(pc.variety, PsyonixBot)

    def test_player_class_human(self):
        pc = PlayerConfiguration(variety=Human())
        assert isinstance(pc.variety, Human)

    def test_player_class_roundtrip(self):
        pc = PlayerConfiguration(
            variety=CustomBot(name="b", root_dir="/tmp", run_command="c", agent_id="a"),
        )
        pc2 = PlayerConfiguration.unpack(pc.pack())
        assert isinstance(pc2.variety, CustomBot)
        assert pc2.variety.name == "b"

    def test_collision_shape_box(self):
        ball = BallInfo(shape=BoxShape(length=100, width=50, height=30))
        assert isinstance(ball.shape, BoxShape)
        assert ball.shape.length == 100

    def test_collision_shape_sphere(self):
        ball = BallInfo(shape=SphereShape(diameter=100))
        assert isinstance(ball.shape, SphereShape)
        assert ball.shape.diameter == 100

    def test_collision_shape_cylinder(self):
        ball = BallInfo(shape=CylinderShape(diameter=50, height=100))
        assert isinstance(ball.shape, CylinderShape)
        assert ball.shape.diameter == 50

    def test_collision_shape_roundtrip(self):
        ball = BallInfo(shape=BoxShape(length=100, width=50, height=30))
        ball2 = BallInfo.unpack(ball.pack())
        assert isinstance(ball2.shape, BoxShape)
        assert ball2.shape.length == 100

    def test_relative_anchor_none(self):
        anchor = RenderAnchor()
        assert anchor.relative is None

    def test_relative_anchor_car(self):
        anchor = RenderAnchor(relative=CarAnchor(0, Vector3(1, 2, 3)))
        assert isinstance(anchor.relative, CarAnchor)
        assert anchor.relative.index == 0
        assert anchor.relative.local.x == 1


class TestFrozenTypes:
    """Frozen types cannot be mutated after creation."""

    @pytest.mark.parametrize(
        "cls, kwargs",
        [
            (PlayerInfo, {"name": "test"}),
            (BallInfo, {"shape": BoxShape()}),
            (
                GamePacket,
                {"players": [], "balls": [], "boost_pads": [], "teams": []},
            ),
            (Physics, {}),
            (Vector3, {"x": 1, "y": 2, "z": 3}),
            (
                Touch,
                {
                    "game_seconds": 1.0,
                    "location": Vector3(),
                    "normal": Vector3(),
                },
            ),
        ],
    )
    def test_frozen_types_cannot_be_mutated(self, cls, kwargs):
        obj = cls(**kwargs)
        with pytest.raises(AttributeError):
            obj.some_field = 1

    def test_frozen_roundtrip(self):
        pi = PlayerInfo(name="test", score_info=ScoreInfo(goals=2))
        pi2 = PlayerInfo.unpack(pi.pack())
        assert pi2.name == "test"
        assert pi2.score_info.goals == 2

    def test_repr(self):
        v = Vector3(x=1.5, y=-2.5, z=10.0)
        s = repr(v)
        assert isinstance(s, str) and len(s) > 0
