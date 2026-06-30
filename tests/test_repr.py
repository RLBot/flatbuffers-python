import pytest

from rlbot_flatbuffers import *


class TestReprSmoke:
    """repr() doesn't crash for any type. Quick smoke test."""

    @pytest.mark.parametrize(
        "name, instance",
        [
            ("AerialGoalScoreMutator", AerialGoalScoreMutator.One),
            ("AirState", AirState.OnGround),
            ("BallAnchor", BallAnchor()),
            ("BallPrediction", BallPrediction()),
            ("BoostPad", BoostPad(location=Vector3())),
            ("BoostPadState", BoostPadState()),
            ("CarAnchor", CarAnchor()),
            ("Color", Color()),
            ("ConnectionSettings", ConnectionSettings("a")),
            ("ConsoleCommand", ConsoleCommand("test")),
            ("ControllableInfo", ControllableInfo()),
            ("ControllableTeamInfo", ControllableTeamInfo(controllables=[])),
            ("ControllerState", ControllerState()),
            ("CylinderShape", CylinderShape()),
            ("DesiredBallState", DesiredBallState()),
            ("DesiredCarState", DesiredCarState()),
            ("DesiredGameState", DesiredGameState()),
            ("DesiredMatchInfo", DesiredMatchInfo()),
            ("DesiredPhysics", DesiredPhysics()),
            ("DisconnectSignal", DisconnectSignal()),
            ("EnvironmentVariable", EnvironmentVariable()),
            ("ExistingMatchBehavior", ExistingMatchBehavior.Restart),
            ("FieldInfo", FieldInfo(boost_pads=[], goals=[], tiles=[])),
            ("GameMode", GameMode.Soccar),
            (
                "GamePacket",
                GamePacket(players=[], balls=[], boost_pads=[], teams=[]),
            ),
            (
                "GoalInfo",
                GoalInfo(location=Vector3(), direction=Vector3()),
            ),
            ("Human", Human()),
            ("InitComplete", InitComplete()),
            ("InvalidFlatbuffer", InvalidFlatbuffer),
            (
                "Line3D",
                Line3D(
                    start=RenderAnchor(),
                    end=RenderAnchor(),
                    color=Color(),
                ),
            ),
            ("LoadoutPaint", LoadoutPaint()),
            ("MatchComm", MatchComm()),
            (
                "MatchConfiguration",
                MatchConfiguration(
                    launcher_arg="",
                    game_map_upk="",
                    player_configurations=[],
                    script_configurations=[],
                ),
            ),
            ("MatchInfo", MatchInfo()),
            ("MatchPhase", MatchPhase.Active),
            ("MutatorSettings", MutatorSettings()),
            ("PerformanceMonitor", PerformanceMonitor.AlwaysShow),
            ("Physics", Physics()),
            ("PingRequest", PingRequest()),
            ("PingResponse", PingResponse()),
            (
                "PlayerConfiguration",
                PlayerConfiguration(variety=Human()),
            ),
            ("PlayerLoadout", PlayerLoadout()),
            ("PolyLine3D", PolyLine3D(points=[], color=Color())),
            ("PredictionSlice", PredictionSlice(game_seconds=0)),
            ("PsyonixBot", PsyonixBot(name="bot")),
            ("PsyonixSkill", PsyonixSkill.Beginner),
            ("Rect2D", Rect2D(color=Color())),
            (
                "Rect3D",
                Rect3D(anchor=RenderAnchor(), color=Color()),
            ),
            ("RemoveRenderGroup", RemoveRenderGroup()),
            ("RenderAnchor", RenderAnchor()),
            ("RenderGroup", RenderGroup(render_messages=[])),
            ("RenderMessage", RenderMessage()),
            ("RenderingStatus", RenderingStatus()),
            ("Rotator", Rotator()),
            ("RotatorPartial", RotatorPartial()),
            ("RumbleItem", RumbleItem),
            ("ScoreInfo", ScoreInfo()),
            (
                "ScriptConfiguration",
                ScriptConfiguration(
                    name="s",
                    root_dir="r",
                    run_command="c",
                    agent_id="a",
                ),
            ),
            ("SetLoadout", SetLoadout(loadout=PlayerLoadout())),
            ("SphereShape", SphereShape()),
            ("StartCommand", StartCommand(config_path="/tmp")),
            ("StopCommand", StopCommand()),
            (
                "String2D",
                String2D(
                    text="hi",
                    foreground=Color(),
                    background=Color(),
                ),
            ),
            (
                "String3D",
                String3D(
                    text="hi",
                    anchor=RenderAnchor(),
                    foreground=Color(),
                    background=Color(),
                ),
            ),
            ("TeamInfo", TeamInfo()),
            ("Tile", Tile(location=Vector3())),
            ("TileDamageLevel", TileDamageLevel.Damaged),
            (
                "Touch",
                Touch(
                    game_seconds=0,
                    location=Vector3(),
                    normal=Vector3(),
                ),
            ),
            ("UpdatePerformanceMonitor", UpdatePerformanceMonitor()),
            ("Vector2", Vector2()),
            ("Vector3Partial", Vector3Partial()),
        ],
    )
    def test_repr(self, name, instance):
        if isinstance(instance, type):
            return  # skip class objects (RumbleItem, InvalidFlatbuffer)
        r = repr(instance)
        assert isinstance(r, str)
        assert len(r) > 0
