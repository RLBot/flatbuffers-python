from __future__ import annotations

from typing import Sequence

__doc__: str
__version__: str

class InvalidFlatbuffer(ValueError): ...

class AerialGoalScoreMutator:
    One: AerialGoalScoreMutator
    """
    `assert int(AerialGoalScoreMutator.One) == 0`
    """
    Zero: AerialGoalScoreMutator
    """
    `assert int(AerialGoalScoreMutator.Zero) == 1`
    """
    Two: AerialGoalScoreMutator
    """
    `assert int(AerialGoalScoreMutator.Two) == 2`
    """
    Three: AerialGoalScoreMutator
    """
    `assert int(AerialGoalScoreMutator.Three) == 3`
    """
    Five: AerialGoalScoreMutator
    """
    `assert int(AerialGoalScoreMutator.Five) == 4`
    """
    Ten: AerialGoalScoreMutator
    """
    `assert int(AerialGoalScoreMutator.Ten) == 5`
    """

    def __new__(cls, value: int = 0) -> AerialGoalScoreMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class AirState:
    """
    Possible states of a car in regards to ground contact and jump/dodging forces.
    See more about jumping physics at https://wiki.rlbot.org/botmaking/jumping-physics/
    """

    OnGround: AirState
    """
    `assert int(AirState.OnGround) == 0`

    All wheels are on the ground and the car is affected by wall-stickiness forces.
    """
    Jumping: AirState
    """
    `assert int(AirState.Jumping) == 1`

    The car is currently affected by jumping forces of an initial jump.
    Lasts until the player lets go of jump button but at most for 0.2 seconds (240 ticks).
    The following AirState is typically InAir.
    """
    DoubleJumping: AirState
    """
    `assert int(AirState.DoubleJumping) == 2`

    The car is currently affected by jumping forces of a secondary jump (just an impulse in practice).
    Lasts for 13 ticks.
    The following AirState is typically InAir.
    """
    Dodging: AirState
    """
    `assert int(AirState.Dodging) == 3`

    The car is currently affected by forces and torque of a dodges.
    This lasts for 79 ticks.
    The following AirState is typically InAir.
    """
    InAir: AirState
    """
    `assert int(AirState.InAir) == 4`

    The car is free falling.
    """

    def __new__(cls, value: int = 0) -> AirState: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class AssistGoalScoreMutator:
    Zero: AssistGoalScoreMutator
    """
    `assert int(AssistGoalScoreMutator.Zero) == 0`
    """
    One: AssistGoalScoreMutator
    """
    `assert int(AssistGoalScoreMutator.One) == 1`
    """
    Two: AssistGoalScoreMutator
    """
    `assert int(AssistGoalScoreMutator.Two) == 2`
    """
    Three: AssistGoalScoreMutator
    """
    `assert int(AssistGoalScoreMutator.Three) == 3`
    """

    def __new__(cls, value: int = 0) -> AssistGoalScoreMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class AudioMutator:
    """
    Audio mutator options.
    """

    Default: AudioMutator
    """
    `assert int(AudioMutator.Default) == 0`
    """
    Haunted: AudioMutator
    """
    `assert int(AudioMutator.Haunted) == 1`
    """

    def __new__(cls, value: int = 0) -> AudioMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallBouncinessMutator:
    """
    Ball bounciness mutator options.
    """

    Default: BallBouncinessMutator
    """
    `assert int(BallBouncinessMutator.Default) == 0`
    """
    Low: BallBouncinessMutator
    """
    `assert int(BallBouncinessMutator.Low) == 1`
    """
    High: BallBouncinessMutator
    """
    `assert int(BallBouncinessMutator.High) == 2`
    """
    SuperHigh: BallBouncinessMutator
    """
    `assert int(BallBouncinessMutator.SuperHigh) == 3`
    """
    Lowish: BallBouncinessMutator
    """
    `assert int(BallBouncinessMutator.Lowish) == 4`
    """

    def __new__(cls, value: int = 0) -> BallBouncinessMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallGravityMutator:
    """
    Ball gravity mutator options.
    """

    Default: BallGravityMutator
    """
    `assert int(BallGravityMutator.Default) == 0`
    """
    Low: BallGravityMutator
    """
    `assert int(BallGravityMutator.Low) == 1`
    """
    High: BallGravityMutator
    """
    `assert int(BallGravityMutator.High) == 2`
    """
    SuperHigh: BallGravityMutator
    """
    `assert int(BallGravityMutator.SuperHigh) == 3`
    """

    def __new__(cls, value: int = 0) -> BallGravityMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallMaxSpeedMutator:
    """
    Ball max speed mutator options.
    """

    Default: BallMaxSpeedMutator
    """
    `assert int(BallMaxSpeedMutator.Default) == 0`
    """
    Slow: BallMaxSpeedMutator
    """
    `assert int(BallMaxSpeedMutator.Slow) == 1`
    """
    Fast: BallMaxSpeedMutator
    """
    `assert int(BallMaxSpeedMutator.Fast) == 2`
    """
    SuperFast: BallMaxSpeedMutator
    """
    `assert int(BallMaxSpeedMutator.SuperFast) == 3`
    """

    def __new__(cls, value: int = 0) -> BallMaxSpeedMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallSizeMutator:
    """
    Ball size mutator options.
    """

    Default: BallSizeMutator
    """
    `assert int(BallSizeMutator.Default) == 0`
    """
    Small: BallSizeMutator
    """
    `assert int(BallSizeMutator.Small) == 1`
    """
    Medium: BallSizeMutator
    """
    `assert int(BallSizeMutator.Medium) == 2`
    """
    Large: BallSizeMutator
    """
    `assert int(BallSizeMutator.Large) == 3`
    """
    Gigantic: BallSizeMutator
    """
    `assert int(BallSizeMutator.Gigantic) == 4`
    """

    def __new__(cls, value: int = 0) -> BallSizeMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallTypeMutator:
    """
    Ball type mutator options.
    """

    Default: BallTypeMutator
    """
    `assert int(BallTypeMutator.Default) == 0`
    """
    Cube: BallTypeMutator
    """
    `assert int(BallTypeMutator.Cube) == 1`
    """
    Puck: BallTypeMutator
    """
    `assert int(BallTypeMutator.Puck) == 2`
    """
    Basketball: BallTypeMutator
    """
    `assert int(BallTypeMutator.Basketball) == 3`
    """
    Beachball: BallTypeMutator
    """
    `assert int(BallTypeMutator.Beachball) == 4`
    """
    Anniversary: BallTypeMutator
    """
    `assert int(BallTypeMutator.Anniversary) == 5`
    """
    Haunted: BallTypeMutator
    """
    `assert int(BallTypeMutator.Haunted) == 6`
    """
    Ekin: BallTypeMutator
    """
    `assert int(BallTypeMutator.Ekin) == 7`
    """
    SpookyCube: BallTypeMutator
    """
    `assert int(BallTypeMutator.SpookyCube) == 8`
    """
    Egg: BallTypeMutator
    """
    `assert int(BallTypeMutator.Egg) == 9`
    """
    PlayerSeeking: BallTypeMutator
    """
    `assert int(BallTypeMutator.PlayerSeeking) == 10`
    """
    Dropshot: BallTypeMutator
    """
    `assert int(BallTypeMutator.Dropshot) == 11`
    """
    ScoreAbsorb: BallTypeMutator
    """
    `assert int(BallTypeMutator.ScoreAbsorb) == 12`
    """
    Shoe: BallTypeMutator
    """
    `assert int(BallTypeMutator.Shoe) == 13`
    """
    PizzaPuck: BallTypeMutator
    """
    `assert int(BallTypeMutator.PizzaPuck) == 14`
    """

    def __new__(cls, value: int = 0) -> BallTypeMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallWeightMutator:
    """
    Ball weight mutator options.
    """

    Default: BallWeightMutator
    """
    `assert int(BallWeightMutator.Default) == 0`
    """
    Light: BallWeightMutator
    """
    `assert int(BallWeightMutator.Light) == 1`
    """
    Heavy: BallWeightMutator
    """
    `assert int(BallWeightMutator.Heavy) == 2`
    """
    SuperLight: BallWeightMutator
    """
    `assert int(BallWeightMutator.SuperLight) == 3`
    """
    CurveBall: BallWeightMutator
    """
    `assert int(BallWeightMutator.CurveBall) == 4`
    """
    BeachBallCurve: BallWeightMutator
    """
    `assert int(BallWeightMutator.BeachBallCurve) == 5`
    """
    MagnusFutBall: BallWeightMutator
    """
    `assert int(BallWeightMutator.MagnusFutBall) == 6`
    """
    MagnusFutballLess: BallWeightMutator
    """
    `assert int(BallWeightMutator.MagnusFutballLess) == 7`
    """

    def __new__(cls, value: int = 0) -> BallWeightMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BoostAmountMutator:
    """
    Boost amount mutator options.
    """

    NormalBoost: BoostAmountMutator
    """
    `assert int(BoostAmountMutator.NormalBoost) == 0`
    """
    UnlimitedBoost: BoostAmountMutator
    """
    `assert int(BoostAmountMutator.UnlimitedBoost) == 1`
    """
    SlowRecharge: BoostAmountMutator
    """
    `assert int(BoostAmountMutator.SlowRecharge) == 2`
    """
    RapidRecharge: BoostAmountMutator
    """
    `assert int(BoostAmountMutator.RapidRecharge) == 3`
    """
    NoBoost: BoostAmountMutator
    """
    `assert int(BoostAmountMutator.NoBoost) == 4`
    """

    def __new__(cls, value: int = 0) -> BoostAmountMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BoostStrengthMutator:
    """
    Boost strength mutator options.
    """

    One: BoostStrengthMutator
    """
    `assert int(BoostStrengthMutator.One) == 0`
    """
    OneAndAHalf: BoostStrengthMutator
    """
    `assert int(BoostStrengthMutator.OneAndAHalf) == 1`
    """
    Two: BoostStrengthMutator
    """
    `assert int(BoostStrengthMutator.Two) == 2`
    """
    Five: BoostStrengthMutator
    """
    `assert int(BoostStrengthMutator.Five) == 3`
    """
    Ten: BoostStrengthMutator
    """
    `assert int(BoostStrengthMutator.Ten) == 4`
    """

    def __new__(cls, value: int = 0) -> BoostStrengthMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DebugRendering:
    OffByDefault: DebugRendering
    """
    `assert int(DebugRendering.OffByDefault) == 0`

    Renders are ignored unless explicitly enabled per-agent
    """
    OnByDefault: DebugRendering
    """
    `assert int(DebugRendering.OnByDefault) == 1`

    Rendering is enabled for everyone by default
    """
    AlwaysOff: DebugRendering
    """
    `assert int(DebugRendering.AlwaysOff) == 2`

    Ignore all render attempts at all times
    """

    def __new__(cls, value: int = 0) -> DebugRendering: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DemolishMutator:
    """
    Demolition mutator options.
    """

    Default: DemolishMutator
    """
    `assert int(DemolishMutator.Default) == 0`
    """
    Disabled: DemolishMutator
    """
    `assert int(DemolishMutator.Disabled) == 1`
    """
    FriendlyFire: DemolishMutator
    """
    `assert int(DemolishMutator.FriendlyFire) == 2`
    """
    OnContact: DemolishMutator
    """
    `assert int(DemolishMutator.OnContact) == 3`
    """
    OnContactFf: DemolishMutator
    """
    `assert int(DemolishMutator.OnContactFf) == 4`
    """
    OnBallContact: DemolishMutator
    """
    `assert int(DemolishMutator.OnBallContact) == 5`
    """
    OnBallContactFf: DemolishMutator
    """
    `assert int(DemolishMutator.OnBallContactFf) == 6`
    """
    OnBallContactSilent: DemolishMutator
    """
    `assert int(DemolishMutator.OnBallContactSilent) == 7`
    """
    OnBallContactFfSilent: DemolishMutator
    """
    `assert int(DemolishMutator.OnBallContactFfSilent) == 8`
    """

    def __new__(cls, value: int = 0) -> DemolishMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DemolishScoreMutator:
    Zero: DemolishScoreMutator
    """
    `assert int(DemolishScoreMutator.Zero) == 0`
    """
    One: DemolishScoreMutator
    """
    `assert int(DemolishScoreMutator.One) == 1`
    """
    Two: DemolishScoreMutator
    """
    `assert int(DemolishScoreMutator.Two) == 2`
    """
    Three: DemolishScoreMutator
    """
    `assert int(DemolishScoreMutator.Three) == 3`
    """

    def __new__(cls, value: int = 0) -> DemolishScoreMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DodgeTimerMutator:
    OnePointTwentyFiveSeconds: DodgeTimerMutator
    """
    `assert int(DodgeTimerMutator.OnePointTwentyFiveSeconds) == 0`
    """
    TwoSeconds: DodgeTimerMutator
    """
    `assert int(DodgeTimerMutator.TwoSeconds) == 1`
    """
    ThreeSeconds: DodgeTimerMutator
    """
    `assert int(DodgeTimerMutator.ThreeSeconds) == 2`
    """
    Unlimited: DodgeTimerMutator
    """
    `assert int(DodgeTimerMutator.Unlimited) == 3`
    """

    def __new__(cls, value: int = 0) -> DodgeTimerMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ExistingMatchBehavior:
    """
    Possible behaviours when a match is started while another match is in progress.
    """

    Restart: ExistingMatchBehavior
    """
    `assert int(ExistingMatchBehavior.Restart) == 0`

    Always restart the match, even if config is identical.
    """
    ContinueAndSpawn: ExistingMatchBehavior
    """
    `assert int(ExistingMatchBehavior.ContinueAndSpawn) == 1`

    Never restart an existing match if possible, just try to remove or spawn cars to match the configuration.
    If we are not in the middle of a match, a match will be started. Handy for LAN matches.
    """
    RestartIfDifferent: ExistingMatchBehavior
    """
    `assert int(ExistingMatchBehavior.RestartIfDifferent) == 2`

    Restart the match if any match settings differ.
    No other otherwise.
    """

    def __new__(cls, value: int = 0) -> ExistingMatchBehavior: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GameEventMutator:
    """
    Game event mutator options.
    """

    Default: GameEventMutator
    """
    `assert int(GameEventMutator.Default) == 0`
    """
    Haunted: GameEventMutator
    """
    `assert int(GameEventMutator.Haunted) == 1`
    """
    Rugby: GameEventMutator
    """
    `assert int(GameEventMutator.Rugby) == 2`
    """

    def __new__(cls, value: int = 0) -> GameEventMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GameMode:
    """
    Various game modes.
    """

    Soccar: GameMode
    """
    `assert int(GameMode.Soccar) == 0`
    """
    Hoops: GameMode
    """
    `assert int(GameMode.Hoops) == 1`
    """
    Dropshot: GameMode
    """
    `assert int(GameMode.Dropshot) == 2`
    """
    Snowday: GameMode
    """
    `assert int(GameMode.Snowday) == 3`
    """
    Rumble: GameMode
    """
    `assert int(GameMode.Rumble) == 4`
    """
    Heatseeker: GameMode
    """
    `assert int(GameMode.Heatseeker) == 5`
    """
    Gridiron: GameMode
    """
    `assert int(GameMode.Gridiron) == 6`
    """
    Knockout: GameMode
    """
    `assert int(GameMode.Knockout) == 7`
    """

    def __new__(cls, value: int = 0) -> GameMode: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GameSpeedMutator:
    """
    Game speed mutator options.
    """

    Default: GameSpeedMutator
    """
    `assert int(GameSpeedMutator.Default) == 0`
    """
    SloMo: GameSpeedMutator
    """
    `assert int(GameSpeedMutator.SloMo) == 1`
    """
    TimeWarp: GameSpeedMutator
    """
    `assert int(GameSpeedMutator.TimeWarp) == 2`
    """

    def __new__(cls, value: int = 0) -> GameSpeedMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GravityMutator:
    """
    Gravity mutator options.
    """

    Default: GravityMutator
    """
    `assert int(GravityMutator.Default) == 0`
    """
    Low: GravityMutator
    """
    `assert int(GravityMutator.Low) == 1`
    """
    High: GravityMutator
    """
    `assert int(GravityMutator.High) == 2`
    """
    SuperHigh: GravityMutator
    """
    `assert int(GravityMutator.SuperHigh) == 3`
    """
    Reverse: GravityMutator
    """
    `assert int(GravityMutator.Reverse) == 4`
    """

    def __new__(cls, value: int = 0) -> GravityMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class InputRestrictionMutator:
    Default: InputRestrictionMutator
    """
    `assert int(InputRestrictionMutator.Default) == 0`
    """
    Backwards: InputRestrictionMutator
    """
    `assert int(InputRestrictionMutator.Backwards) == 1`
    """

    def __new__(cls, value: int = 0) -> InputRestrictionMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class JumpMutator:
    Default: JumpMutator
    """
    `assert int(JumpMutator.Default) == 0`
    """
    Grounded: JumpMutator
    """
    `assert int(JumpMutator.Grounded) == 1`
    """
    Two: JumpMutator
    """
    `assert int(JumpMutator.Two) == 2`
    """
    Three: JumpMutator
    """
    `assert int(JumpMutator.Three) == 3`
    """
    Four: JumpMutator
    """
    `assert int(JumpMutator.Four) == 4`
    """
    Unlimited: JumpMutator
    """
    `assert int(JumpMutator.Unlimited) == 5`
    """
    NoJumps: JumpMutator
    """
    `assert int(JumpMutator.NoJumps) == 6`
    """

    def __new__(cls, value: int = 0) -> JumpMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Launcher:
    """
    Possible to launch Rocket League.
    """

    Steam: Launcher
    """
    `assert int(Launcher.Steam) == 0`
    """
    Epic: Launcher
    """
    `assert int(Launcher.Epic) == 1`
    """
    Custom: Launcher
    """
    `assert int(Launcher.Custom) == 2`

    E.g. if you use Legendary.
    The game path is specified in the MatchConfiguration.
    """
    NoLaunch: Launcher
    """
    `assert int(Launcher.NoLaunch) == 3`
    """

    def __new__(cls, value: int = 0) -> Launcher: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MatchLengthMutator:
    """
    Match length mutator options.
    """

    FiveMinutes: MatchLengthMutator
    """
    `assert int(MatchLengthMutator.FiveMinutes) == 0`
    """
    TenMinutes: MatchLengthMutator
    """
    `assert int(MatchLengthMutator.TenMinutes) == 1`
    """
    TwentyMinutes: MatchLengthMutator
    """
    `assert int(MatchLengthMutator.TwentyMinutes) == 2`
    """
    Unlimited: MatchLengthMutator
    """
    `assert int(MatchLengthMutator.Unlimited) == 3`
    """

    def __new__(cls, value: int = 0) -> MatchLengthMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MatchPhase:
    """
    Possible phases of the match.
    """

    Inactive: MatchPhase
    """
    `assert int(MatchPhase.Inactive) == 0`

    Match has not been created yet.
    """
    Countdown: MatchPhase
    """
    `assert int(MatchPhase.Countdown) == 1`

    3-2-1 countdown of a kickoff.
    """
    Kickoff: MatchPhase
    """
    `assert int(MatchPhase.Kickoff) == 2`

    After kickoff countdown, but before ball has been hit.
    The match automatically proceeds to Active after 2 seconds.
    """
    Active: MatchPhase
    """
    `assert int(MatchPhase.Active) == 3`

    The ball is in play and time is ticking.
    """
    GoalScored: MatchPhase
    """
    `assert int(MatchPhase.GoalScored) == 4`

    A goal was just scored. Waiting for replay to start.
    """
    Replay: MatchPhase
    """
    `assert int(MatchPhase.Replay) == 5`

    Goal replay is being shown.
    """
    Paused: MatchPhase
    """
    `assert int(MatchPhase.Paused) == 6`

    The match is paused.
    """
    Ended: MatchPhase
    """
    `assert int(MatchPhase.Ended) == 7`

    The match has ended.
    """

    def __new__(cls, value: int = 0) -> MatchPhase: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MaxScoreMutator:
    """
    Max score mutator options.
    """

    Unlimited: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.Unlimited) == 0`
    """
    OneGoal: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.OneGoal) == 1`
    """
    ThreeGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.ThreeGoals) == 2`
    """
    FiveGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.FiveGoals) == 3`
    """
    SevenGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.SevenGoals) == 4`
    """
    TenGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.TenGoals) == 5`
    """
    TwentyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.TwentyGoals) == 6`
    """
    ThirtyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.ThirtyGoals) == 7`
    """
    FortyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.FortyGoals) == 8`
    """
    FiftyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.FiftyGoals) == 9`
    """
    SixtyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.SixtyGoals) == 10`
    """
    SeventyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.SeventyGoals) == 11`
    """
    EightyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.EightyGoals) == 12`
    """
    NinetyGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.NinetyGoals) == 13`
    """
    HundredGoals: MaxScoreMutator
    """
    `assert int(MaxScoreMutator.HundredGoals) == 14`
    """

    def __new__(cls, value: int = 0) -> MaxScoreMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MaxTimeMutator:
    """
    Max time mutator options.
    """

    Unlimited: MaxTimeMutator
    """
    `assert int(MaxTimeMutator.Unlimited) == 0`
    """
    ElevenMinutes: MaxTimeMutator
    """
    `assert int(MaxTimeMutator.ElevenMinutes) == 1`
    """

    def __new__(cls, value: int = 0) -> MaxTimeMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MultiBallMutator:
    """
    Multi ball mutator options.
    """

    One: MultiBallMutator
    """
    `assert int(MultiBallMutator.One) == 0`
    """
    Two: MultiBallMutator
    """
    `assert int(MultiBallMutator.Two) == 1`
    """
    Four: MultiBallMutator
    """
    `assert int(MultiBallMutator.Four) == 2`
    """
    Six: MultiBallMutator
    """
    `assert int(MultiBallMutator.Six) == 3`
    """

    def __new__(cls, value: int = 0) -> MultiBallMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class NormalGoalScoreMutator:
    One: NormalGoalScoreMutator
    """
    `assert int(NormalGoalScoreMutator.One) == 0`
    """
    Zero: NormalGoalScoreMutator
    """
    `assert int(NormalGoalScoreMutator.Zero) == 1`
    """
    Two: NormalGoalScoreMutator
    """
    `assert int(NormalGoalScoreMutator.Two) == 2`
    """
    Three: NormalGoalScoreMutator
    """
    `assert int(NormalGoalScoreMutator.Three) == 3`
    """
    Five: NormalGoalScoreMutator
    """
    `assert int(NormalGoalScoreMutator.Five) == 4`
    """
    Ten: NormalGoalScoreMutator
    """
    `assert int(NormalGoalScoreMutator.Ten) == 5`
    """

    def __new__(cls, value: int = 0) -> NormalGoalScoreMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class OvertimeMutator:
    """
    Overtime mutator options.
    """

    Unlimited: OvertimeMutator
    """
    `assert int(OvertimeMutator.Unlimited) == 0`
    """
    FiveMaxFirstScore: OvertimeMutator
    """
    `assert int(OvertimeMutator.FiveMaxFirstScore) == 1`
    """
    FiveMaxRandomTeam: OvertimeMutator
    """
    `assert int(OvertimeMutator.FiveMaxRandomTeam) == 2`
    """

    def __new__(cls, value: int = 0) -> OvertimeMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PossessionScoreMutator:
    Off: PossessionScoreMutator
    """
    `assert int(PossessionScoreMutator.Off) == 0`
    """
    OneSecond: PossessionScoreMutator
    """
    `assert int(PossessionScoreMutator.OneSecond) == 1`
    """
    TwoSeconds: PossessionScoreMutator
    """
    `assert int(PossessionScoreMutator.TwoSeconds) == 2`
    """
    ThreeSeconds: PossessionScoreMutator
    """
    `assert int(PossessionScoreMutator.ThreeSeconds) == 3`
    """

    def __new__(cls, value: int = 0) -> PossessionScoreMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PsyonixSkill:
    """
    Various skill levels of Psyonix bots.
    """

    Beginner: PsyonixSkill
    """
    `assert int(PsyonixSkill.Beginner) == 0`
    """
    Rookie: PsyonixSkill
    """
    `assert int(PsyonixSkill.Rookie) == 1`
    """
    Pro: PsyonixSkill
    """
    `assert int(PsyonixSkill.Pro) == 2`
    """
    AllStar: PsyonixSkill
    """
    `assert int(PsyonixSkill.AllStar) == 3`
    """

    def __new__(cls, value: int = 0) -> PsyonixSkill: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RespawnTimeMutator:
    """
    Respawn time mutator options.
    """

    ThreeSeconds: RespawnTimeMutator
    """
    `assert int(RespawnTimeMutator.ThreeSeconds) == 0`
    """
    TwoSeconds: RespawnTimeMutator
    """
    `assert int(RespawnTimeMutator.TwoSeconds) == 1`
    """
    OneSecond: RespawnTimeMutator
    """
    `assert int(RespawnTimeMutator.OneSecond) == 2`
    """
    DisableGoalReset: RespawnTimeMutator
    """
    `assert int(RespawnTimeMutator.DisableGoalReset) == 3`
    """

    def __new__(cls, value: int = 0) -> RespawnTimeMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RumbleMutator:
    """
    Rumble mutator options.
    """

    Off: RumbleMutator
    """
    `assert int(RumbleMutator.Off) == 0`
    """
    DefaultRumble: RumbleMutator
    """
    `assert int(RumbleMutator.DefaultRumble) == 1`
    """
    Slow: RumbleMutator
    """
    `assert int(RumbleMutator.Slow) == 2`
    """
    Civilized: RumbleMutator
    """
    `assert int(RumbleMutator.Civilized) == 3`
    """
    DestructionDerby: RumbleMutator
    """
    `assert int(RumbleMutator.DestructionDerby) == 4`
    """
    SpringLoaded: RumbleMutator
    """
    `assert int(RumbleMutator.SpringLoaded) == 5`
    """
    SpikesOnly: RumbleMutator
    """
    `assert int(RumbleMutator.SpikesOnly) == 6`
    """
    SpikeRush: RumbleMutator
    """
    `assert int(RumbleMutator.SpikeRush) == 7`
    """
    HauntedBallBeam: RumbleMutator
    """
    `assert int(RumbleMutator.HauntedBallBeam) == 8`
    """
    Tactical: RumbleMutator
    """
    `assert int(RumbleMutator.Tactical) == 9`
    """
    BatmanRumble: RumbleMutator
    """
    `assert int(RumbleMutator.BatmanRumble) == 10`
    """
    GrapplingOnly: RumbleMutator
    """
    `assert int(RumbleMutator.GrapplingOnly) == 11`
    """
    HaymakerOnly: RumbleMutator
    """
    `assert int(RumbleMutator.HaymakerOnly) == 12`
    """
    SpikeRushForce: RumbleMutator
    """
    `assert int(RumbleMutator.SpikeRushForce) == 13`
    """
    Rps: RumbleMutator
    """
    `assert int(RumbleMutator.Rps) == 14`
    """

    def __new__(cls, value: int = 0) -> RumbleMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ScoringRuleMutator:
    Default: ScoringRuleMutator
    """
    `assert int(ScoringRuleMutator.Default) == 0`
    """
    Disabled: ScoringRuleMutator
    """
    `assert int(ScoringRuleMutator.Disabled) == 1`
    """

    def __new__(cls, value: int = 0) -> ScoringRuleMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class SeriesLengthMutator:
    """
    Series length mutator options.
    """

    Unlimited: SeriesLengthMutator
    """
    `assert int(SeriesLengthMutator.Unlimited) == 0`
    """
    ThreeGames: SeriesLengthMutator
    """
    `assert int(SeriesLengthMutator.ThreeGames) == 1`
    """
    FiveGames: SeriesLengthMutator
    """
    `assert int(SeriesLengthMutator.FiveGames) == 2`
    """
    SevenGames: SeriesLengthMutator
    """
    `assert int(SeriesLengthMutator.SevenGames) == 3`
    """

    def __new__(cls, value: int = 0) -> SeriesLengthMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class StaleBallMutator:
    Unlimited: StaleBallMutator
    """
    `assert int(StaleBallMutator.Unlimited) == 0`
    """
    ThirtySeconds: StaleBallMutator
    """
    `assert int(StaleBallMutator.ThirtySeconds) == 1`
    """

    def __new__(cls, value: int = 0) -> StaleBallMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class TerritoryMutator:
    Off: TerritoryMutator
    """
    `assert int(TerritoryMutator.Off) == 0`
    """
    Territory: TerritoryMutator
    """
    `assert int(TerritoryMutator.Territory) == 1`
    """

    def __new__(cls, value: int = 0) -> TerritoryMutator: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class TextHAlign:
    """
    Horizontal text alignment.
    """

    Left: TextHAlign
    """
    `assert int(TextHAlign.Left) == 0`
    """
    Center: TextHAlign
    """
    `assert int(TextHAlign.Center) == 1`
    """
    Right: TextHAlign
    """
    `assert int(TextHAlign.Right) == 2`
    """

    def __new__(cls, value: int = 0) -> TextHAlign: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class TextVAlign:
    """
    Vertical text alignment.
    """

    Top: TextVAlign
    """
    `assert int(TextVAlign.Top) == 0`
    """
    Center: TextVAlign
    """
    `assert int(TextVAlign.Center) == 1`
    """
    Bottom: TextVAlign
    """
    `assert int(TextVAlign.Bottom) == 2`
    """

    def __new__(cls, value: int = 0) -> TextVAlign: ...
    def __init__(self, value: int = 0) -> None:
        """
        :raises ValueError: If the `value` is not a valid enum value
        """
    def __int__(self) -> int: ...
    def __eq__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BoostPadState:
    """
    The state of a boost pad.
    Note, static properties of boost pads, such as their location and size, are found in the field info.
    """

    is_active: bool
    """
    True if the boost can be picked up right now.
    """
    timer: float
    """
    The number of seconds since the boost has been picked up, or 0 if the boost is active.
    A big boost pad becomes active again after 10 seconds.
    A small boost pad becomes active again after 4 seconds.
    """

    __match_args__ = (
        "is_active",
        "timer",
    )

    def __new__(
        cls,
        is_active: bool = False,
        timer: float = 0.0,
    ) -> BoostPadState: ...
    def __init__(
        self,
        is_active: bool = False,
        timer: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Color:
    """
    An RGBA color.
    """

    r: int
    g: int
    b: int
    a: int

    __match_args__ = (
        "r",
        "g",
        "b",
        "a",
    )

    def __new__(
        cls,
        r: int = 0,
        g: int = 0,
        b: int = 0,
        a: int = 255,
    ) -> Color: ...
    def __init__(
        self,
        r: int = 0,
        g: int = 0,
        b: int = 0,
        a: int = 255,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ControllerState:
    """
    A combination of button presses and analog steering values like those produced by a physical controller or keyboard.
    This is sent by bots each tick to RLBot to indicate what they want to do that tick.
    For example, if you want to hold the jump button for 20 ticks, then you must send 20 controller states where jump is true.
    Remember to send controller states with jump set to false to let go of the jump button afterwards.
    """

    throttle: float
    """
    -1 for full reverse, 1 for full forward.
    """
    steer: float
    """
    -1 for full left, 1 for full right.
    """
    pitch: float
    """
    -1 for nose down, 1 for nose up.
    """
    yaw: float
    """
    -1 for full left, 1 for full right.
    """
    roll: float
    """
    -1 for roll left, 1 for roll right.
    """
    jump: bool
    """
    True if you want to press the jump button.
    """
    boost: bool
    """
    True if you want to press the boost button.
    """
    handbrake: bool
    """
    True if you want to press the handbrake button.
    """
    use_item: bool
    """
    True if you want to press the 'use item' button. Used in Rumble and other game modes.
    """

    __match_args__ = (
        "throttle",
        "steer",
        "pitch",
        "yaw",
        "roll",
        "jump",
        "boost",
        "handbrake",
        "use_item",
    )

    def __new__(
        cls,
        throttle: float = 0.0,
        steer: float = 0.0,
        pitch: float = 0.0,
        yaw: float = 0.0,
        roll: float = 0.0,
        jump: bool = False,
        boost: bool = False,
        handbrake: bool = False,
        use_item: bool = False,
    ) -> ControllerState: ...
    def __init__(
        self,
        throttle: float = 0.0,
        steer: float = 0.0,
        pitch: float = 0.0,
        yaw: float = 0.0,
        roll: float = 0.0,
        jump: bool = False,
        boost: bool = False,
        handbrake: bool = False,
        use_item: bool = False,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Float:
    """
    A floating point value located in a separate struct allowing for optional floats elsewhere.
    """

    val: float

    __match_args__ = (
        "val",
    )

    def __new__(
        cls,
        val: float = 0.0,
    ) -> Float: ...
    def __init__(
        self,
        val: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Rotator:
    """
    Expresses the rotation state of an object in Euler angles. Values are in radians.
    """

    pitch: float
    """
    In radians with range (-pi/2,+pi/2) where 0 is flat, +pi/2 is nose straight up, -pi/2 is nose straight down.
    """
    yaw: float
    """
    In radians with range [-pi,+pi) where 0 is towards positive x, rotating clockwise as increased (when seen from above).
    """
    roll: float
    """
    In radians with range (-pi,+pi) where 0 is upright, positive is tilted right, negative is tilted left.
    """

    __match_args__ = (
        "pitch",
        "yaw",
        "roll",
    )

    def __new__(
        cls,
        pitch: float = 0.0,
        yaw: float = 0.0,
        roll: float = 0.0,
    ) -> Rotator: ...
    def __init__(
        self,
        pitch: float = 0.0,
        yaw: float = 0.0,
        roll: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ScoreInfo:
    """
    A collection of values shown on the scoreboard (and a few more).
    """

    score: int
    """
    The accumulated score, roughly indicating how well a player performs.
    """
    goals: int
    """
    Number of goals scored.
    """
    own_goals: int
    """
    Number of own-goals scored.
    """
    assists: int
    """
    Number of goals assisted.
    """
    saves: int
    """
    Number of shots saved.
    """
    shots: int
    """
    Number of shots on opponent goal.
    """
    demolitions: int
    """
    Number of demolitions made.
    """

    __match_args__ = (
        "score",
        "goals",
        "own_goals",
        "assists",
        "saves",
        "shots",
        "demolitions",
    )

    def __new__(
        cls,
        score: int = 0,
        goals: int = 0,
        own_goals: int = 0,
        assists: int = 0,
        saves: int = 0,
        shots: int = 0,
        demolitions: int = 0,
    ) -> ScoreInfo: ...
    def __init__(
        self,
        score: int = 0,
        goals: int = 0,
        own_goals: int = 0,
        assists: int = 0,
        saves: int = 0,
        shots: int = 0,
        demolitions: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class TeamInfo:
    """
    Information about teams. Currently only the number of goals scored.
    """

    team_index: int
    """
    The index of the team. Blue is 0, orange is 1.
    """
    score: int
    """
    Number of goals scored.
    Note, this value may be different than the sum of the goals scored by the current players on the team as player may join/leave the game or switch teams.
    This value is what is shown on the heads-up display.
    """

    __match_args__ = (
        "team_index",
        "score",
    )

    def __new__(
        cls,
        team_index: int = 0,
        score: int = 0,
    ) -> TeamInfo: ...
    def __init__(
        self,
        team_index: int = 0,
        score: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Vector2:
    """
    A vector with an x and y component.
    """

    x: float
    y: float

    __match_args__ = (
        "x",
        "y",
    )

    def __new__(
        cls,
        x: float = 0.0,
        y: float = 0.0,
    ) -> Vector2: ...
    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Vector3:
    """
    A vector with an x, y, z component.
    Note that Rocket League uses a left-handed coordinate system.
    """

    x: float
    y: float
    z: float

    __match_args__ = (
        "x",
        "y",
        "z",
    )

    def __new__(
        cls,
        x: float = 0.0,
        y: float = 0.0,
        z: float = 0.0,
    ) -> Vector3: ...
    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        z: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Physics:
    """
    The physical state of an object.
    """

    location: Vector3
    rotation: Rotator
    velocity: Vector3
    angular_velocity: Vector3

    __match_args__ = (
        "location",
        "rotation",
        "velocity",
        "angular_velocity",
    )

    def __new__(
        cls,
        location: Vector3 = Vector3(),
        rotation: Rotator = Rotator(),
        velocity: Vector3 = Vector3(),
        angular_velocity: Vector3 = Vector3(),
    ) -> Physics: ...
    def __init__(
        self,
        location: Vector3 = Vector3(),
        rotation: Rotator = Rotator(),
        velocity: Vector3 = Vector3(),
        angular_velocity: Vector3 = Vector3(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PredictionSlice:
    """
    An entry in the ball prediction describing where a ball will be at some future time.
    """

    game_seconds: float
    """
    The moment in game time that this prediction corresponds to.
    This corresponds to 'seconds_elapsed' in the MatchInfo.
    """
    physics: Physics
    """
    The predicted location and motion of the object.
    """

    __match_args__ = (
        "game_seconds",
        "physics",
    )

    def __new__(
        cls,
        game_seconds: float = 0.0,
        physics: Physics = Physics(),
    ) -> PredictionSlice: ...
    def __init__(
        self,
        game_seconds: float = 0.0,
        physics: Physics = Physics(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PlayerLoadout:
    """
    Defines the car type, color, and other aspects of the player's appearance.
    See https://wiki.rlbot.org/botmaking/bot-customization/
    """

    team_color_id: int
    custom_color_id: int
    car_id: int
    decal_id: int
    wheels_id: int
    boost_id: int
    antenna_id: int
    hat_id: int
    paint_finish_id: int
    custom_finish_id: int
    engine_audio_id: int
    trails_id: int
    goal_explosion_id: int
    loadout_paint: LoadoutPaint | None
    primary_color_lookup: Color | None
    """
    Sets the primary color of the car to the swatch that most closely matches the provided
    RGB color value. If set, this overrides teamColorId.
    """
    secondary_color_lookup: Color | None
    """
    Sets the secondary color of the car to the swatch that most closely matches the provided
    RGB color value. If set, this overrides customColorId.
    """

    __match_args__ = (
        "team_color_id",
        "custom_color_id",
        "car_id",
        "decal_id",
        "wheels_id",
        "boost_id",
        "antenna_id",
        "hat_id",
        "paint_finish_id",
        "custom_finish_id",
        "engine_audio_id",
        "trails_id",
        "goal_explosion_id",
        "loadout_paint",
        "primary_color_lookup",
        "secondary_color_lookup",
    )

    def __new__(
        cls,
        team_color_id: int = 0,
        custom_color_id: int = 0,
        car_id: int = 0,
        decal_id: int = 0,
        wheels_id: int = 0,
        boost_id: int = 0,
        antenna_id: int = 0,
        hat_id: int = 0,
        paint_finish_id: int = 0,
        custom_finish_id: int = 0,
        engine_audio_id: int = 0,
        trails_id: int = 0,
        goal_explosion_id: int = 0,
        loadout_paint: LoadoutPaint | None = None,
        primary_color_lookup: Color | None = None,
        secondary_color_lookup: Color | None = None,
    ) -> PlayerLoadout: ...
    def __init__(
        self,
        team_color_id: int = 0,
        custom_color_id: int = 0,
        car_id: int = 0,
        decal_id: int = 0,
        wheels_id: int = 0,
        boost_id: int = 0,
        antenna_id: int = 0,
        hat_id: int = 0,
        paint_finish_id: int = 0,
        custom_finish_id: int = 0,
        engine_audio_id: int = 0,
        trails_id: int = 0,
        goal_explosion_id: int = 0,
        loadout_paint: LoadoutPaint | None = None,
        primary_color_lookup: Color | None = None,
        secondary_color_lookup: Color | None = None,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PlayerLoadout:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RenderAnchor:
    """
    A RenderAnchor is a point in space consisting of a world component and optionally a relative component.
    The relative component is given by a car or ball and includes a local offset that takes the orientation of the object into account.
    The RenderAnchor stays attached to the object and does not have to be updated each tick.
    Rendering that uses a RenderAnchor attached to an object disappears if the object is destroyed, i.e. the car demolished or the ball is scored.
    """

    world: Vector3
    """
    An offset in global coordinates.
    If the relative component is null, then this simply a point in 3D space.
    """
    relative: BallAnchor | CarAnchor | None
    """
    An optional offset given by the position of an object and includes a local offset that takes the object's orientation into account.
    """

    __match_args__ = (
        "world",
        "relative",
    )

    def __new__(
        cls,
        world: Vector3 = Vector3(),
        relative: BallAnchor | CarAnchor | None = None,
    ) -> RenderAnchor: ...
    def __init__(
        self,
        world: Vector3 = Vector3(),
        relative: BallAnchor | CarAnchor | None = None,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> RenderAnchor:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BoxShape:
    """
    A box with dimensions.
    Used for cars and balls with a box shapes.
    """

    length: float
    width: float
    height: float

    __match_args__ = (
        "length",
        "width",
        "height",
    )

    def __new__(
        cls,
        length: float = 0.0,
        width: float = 0.0,
        height: float = 0.0,
    ) -> BoxShape: ...
    def __init__(
        self,
        length: float = 0.0,
        width: float = 0.0,
        height: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> BoxShape:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DisconnectSignal:
    """
    Sent to core to indicate that you want to disconnect.
    Sent from core to indicate that you should exit.
    """

    def __new__(cls) -> DisconnectSignal: ...
    def __init__(self) -> None: ...

class MatchComm:
    """
    A message sent to other bots and scripts through RLBot.
    Use the team_only field if the message should only be received by team mates.
    The message can also have a human-readable summary displayed in quick chat through the display field.
    """

    index: int
    """
    The index of the player that sent this message.
    For scripts, this value is the index in the match configuration instead.
    """
    team: int
    """
    The team of the player that sent this message.
    For scripts, this value is 2.
    """
    team_only: bool
    """
    True if this message is team-only, false if everyone can see it
    """
    display: str | None
    """
    The message that will be displayed on the screen in quick chat.
    This is intended for communication with humans.
    Use the content field for communication with bots and scripts.
    """
    content: bytes
    """
    The contents of the message.
    Use the display field for messages in quick chat.
    """

    __match_args__ = (
        "index",
        "team",
        "team_only",
        "display",
        "content",
    )

    def __new__(
        cls,
        index: int = 0,
        team: int = 0,
        team_only: bool = False,
        display: str | None = None,
        content: bytes = bytes(),
    ) -> MatchComm: ...
    def __init__(
        self,
        index: int = 0,
        team: int = 0,
        team_only: bool = False,
        display: str | None = None,
        content: bytes = bytes(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> MatchComm:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PingRequest:
    """
    Ping request message
    """

    cookie: int

    __match_args__ = (
        "cookie",
    )

    def __new__(
        cls,
        cookie: int = 0,
    ) -> PingRequest: ...
    def __init__(
        self,
        cookie: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PingRequest:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PingResponse:
    """
    Ping response message
    """

    cookie: int

    __match_args__ = (
        "cookie",
    )

    def __new__(
        cls,
        cookie: int = 0,
    ) -> PingResponse: ...
    def __init__(
        self,
        cookie: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PingResponse:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RenderingStatus:
    """
    As an interface message, this requests for a specificed agent to have its ability to render changed.
    This changed will then be broadcasted to all current connections as a core message.
    Does nothing if rendering has been completely disabled.
    """

    index: int
    """
    If `is_bot`, this is the index of the bot in `GamePacket` that has been updated.
    Otherwise, this is the index of the script in `MatchConfiguration` that has been updated.
    """
    is_bot: bool
    """
    Identifies if the index is that of a bot or a script
    """
    status: bool
    """
    If rendering is now enabled or disabled for the specific agent
    """

    __match_args__ = (
        "index",
        "is_bot",
        "status",
    )

    def __new__(
        cls,
        index: int = 0,
        is_bot: bool = False,
        status: bool = False,
    ) -> RenderingStatus: ...
    def __init__(
        self,
        index: int = 0,
        is_bot: bool = False,
        status: bool = False,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> RenderingStatus:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DesiredPhysics:
    """
    A physical state of an object, with nullable components.
    Used for game state setting to define which part of a physics body should change.
    If a component is null, then the component will keep its current value.
    """

    location: Vector3Partial | None
    rotation: RotatorPartial | None
    velocity: Vector3Partial | None
    angular_velocity: Vector3Partial | None

    __match_args__ = (
        "location",
        "rotation",
        "velocity",
        "angular_velocity",
    )

    def __new__(
        cls,
        location: Vector3Partial | None = None,
        rotation: RotatorPartial | None = None,
        velocity: Vector3Partial | None = None,
        angular_velocity: Vector3Partial | None = None,
    ) -> DesiredPhysics: ...
    def __init__(
        self,
        location: Vector3Partial | None = None,
        rotation: RotatorPartial | None = None,
        velocity: Vector3Partial | None = None,
        angular_velocity: Vector3Partial | None = None,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> DesiredPhysics:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MatchConfiguration:
    """
    Definition of a match.
    Can be sent to RLBot to request the start of a match.
    """

    launcher: Launcher
    """
    How to launch Rocket League.
    If left unset, RLBot will not launch the game.
    To use Legendary, use Custom and set launcher_arg="legendary".
    """
    launcher_arg: str
    """
    Additional configuration for the launching method.
    See launcher.
    """
    auto_start_agents: bool
    """
    If true, RLBot will start the bots and scripts that has a non-empty run command in their player/script configuration.
    """
    wait_for_agents: bool
    """
    If true, RLBot will start the match only once all bots and script have connected and are ready.
    If false, the match will start as soon as the map loads.
    """
    game_map_upk: str
    """
    The name of a upk file, like UtopiaStadium_P, which should be loaded.
    On Steam version of Rocket League this can be used to load custom map files,
    but on Epic version it only works on the Psyonix maps.
    Available maps can be found here: https://github.com/VirxEC/python-interface/blob/master/rlbot/utils/maps.py
    """
    player_configurations: Sequence[PlayerConfiguration]
    """
    The players in the match.
    """
    script_configurations: Sequence[ScriptConfiguration]
    """
    The custom scripts used in the match.
    """
    game_mode: GameMode
    """
    The game mode.
    This affects a few of the game rules although many game modes can also be recreated solely from mutators.
    See what mutators and game mode combinations make up the official modes at https://github.com/VirxEC/python-interface/tree/master/tests/gamemodes
    """
    skip_replays: bool
    """
    Whether to skip goal replays.
    """
    instant_start: bool
    """
    Whether to start without a kickoff countdown.
    """
    mutators: MutatorSettings | None
    """
    Mutator settings.
    """
    existing_match_behavior: ExistingMatchBehavior
    """
    How to handle any ongoing match.
    """
    enable_rendering: DebugRendering
    """
    Whether debug rendering is displayed.
    """
    enable_state_setting: bool
    """
    Whether clients are allowed to manipulate the game state, e.g. teleporting cars and ball.
    """
    auto_save_replay: bool
    """
    Whether the match replay should be saved.
    """
    freeplay: bool
    """
    If set to true, a free play match is launched instead of an exhibition match.
    This allows the players to use training keybinds, Bakkesmod plugins, and other features that are only allowed in free play.
    """

    __match_args__ = (
        "launcher",
        "launcher_arg",
        "auto_start_agents",
        "wait_for_agents",
        "game_map_upk",
        "player_configurations",
        "script_configurations",
        "game_mode",
        "skip_replays",
        "instant_start",
        "mutators",
        "existing_match_behavior",
        "enable_rendering",
        "enable_state_setting",
        "auto_save_replay",
        "freeplay",
    )

    def __new__(
        cls,
        launcher: Launcher = Launcher(),
        launcher_arg: str = "",
        auto_start_agents: bool = False,
        wait_for_agents: bool = False,
        game_map_upk: str = "",
        player_configurations: Sequence[PlayerConfiguration] = [],
        script_configurations: Sequence[ScriptConfiguration] = [],
        game_mode: GameMode = GameMode(),
        skip_replays: bool = False,
        instant_start: bool = False,
        mutators: MutatorSettings | None = None,
        existing_match_behavior: ExistingMatchBehavior = ExistingMatchBehavior(),
        enable_rendering: DebugRendering = DebugRendering(),
        enable_state_setting: bool = False,
        auto_save_replay: bool = False,
        freeplay: bool = False,
    ) -> MatchConfiguration: ...
    def __init__(
        self,
        launcher: Launcher = Launcher(),
        launcher_arg: str = "",
        auto_start_agents: bool = False,
        wait_for_agents: bool = False,
        game_map_upk: str = "",
        player_configurations: Sequence[PlayerConfiguration] = [],
        script_configurations: Sequence[ScriptConfiguration] = [],
        game_mode: GameMode = GameMode(),
        skip_replays: bool = False,
        instant_start: bool = False,
        mutators: MutatorSettings | None = None,
        existing_match_behavior: ExistingMatchBehavior = ExistingMatchBehavior(),
        enable_rendering: DebugRendering = DebugRendering(),
        enable_state_setting: bool = False,
        auto_save_replay: bool = False,
        freeplay: bool = False,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> MatchConfiguration:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ConnectionSettings:
    """
    Sent by clients when connecting to RLBot to indicate what type of messages are desired.
    This could be sent by a bot, or a bot manager governing several bots, an
    overlay, or any other utility that connects to the RLBot process.
    """

    agent_id: str
    """
    The ID of the bot/script that is associated with the incoming connection.
    """
    wants_ball_predictions: bool
    """
    If this is set, RLBot will send BallPrediction data back to the client when available.
    """
    wants_comms: bool
    """
    If this is set, RLBot will send MatchComms to the client when available.
    """
    close_between_matches: bool
    """
    If this is set, RLBot will close the connection when a match is stopped or when a new
    match is started. The GUI and other match runners should likely not set this.
    """

    __match_args__ = (
        "agent_id",
        "wants_ball_predictions",
        "wants_comms",
        "close_between_matches",
    )

    def __new__(
        cls,
        agent_id: str = "",
        wants_ball_predictions: bool = False,
        wants_comms: bool = False,
        close_between_matches: bool = False,
    ) -> ConnectionSettings: ...
    def __init__(
        self,
        agent_id: str = "",
        wants_ball_predictions: bool = False,
        wants_comms: bool = False,
        close_between_matches: bool = False,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> ConnectionSettings:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ConsoleCommand:
    """
    A console command which will be executed inside Rocket League.
    See https://wiki.rlbot.org/framework/console-commands/ for a list of known commands.
    """

    command: str

    __match_args__ = (
        "command",
    )

    def __new__(
        cls,
        command: str = "",
    ) -> ConsoleCommand: ...
    def __init__(
        self,
        command: str = "",
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> ConsoleCommand:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ControllableInfo:
    """
    Information about a car that the client can control.
    """

    index: int
    """
    The index of the bot/script.
    """
    identifier: int
    """
    The id of the bot/script.
    This value is mostly used internally to keep track of participants in the match.
    The id can be used to find the corresponding PlayerConfiguration in the MatchConfiguration.
    """

    __match_args__ = (
        "index",
        "identifier",
    )

    def __new__(
        cls,
        index: int = 0,
        identifier: int = 0,
    ) -> ControllableInfo: ...
    def __init__(
        self,
        index: int = 0,
        identifier: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> ControllableInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class CylinderShape:
    """
    A cylinder with diameter and height.
    Used for balls with a cylindrical shape like the puck.
    """

    diameter: float
    height: float

    __match_args__ = (
        "diameter",
        "height",
    )

    def __new__(
        cls,
        diameter: float = 0.0,
        height: float = 0.0,
    ) -> CylinderShape: ...
    def __init__(
        self,
        diameter: float = 0.0,
        height: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> CylinderShape:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Human:
    """
    A normal human player.
    """

    def __new__(cls) -> Human: ...
    def __init__(self) -> None: ...

class InitComplete:
    """
    Indicates that the session has finished all initialization and is ready to start receiving
    game messages without delay.
    """

    def __new__(cls) -> InitComplete: ...
    def __init__(self) -> None: ...

class LoadoutPaint:
    """
    Specification for 'painted' items. See https://wiki.rlbot.org/botmaking/bot-customization/
    """

    car_paint_id: int
    decal_paint_id: int
    wheels_paint_id: int
    boost_paint_id: int
    antenna_paint_id: int
    hat_paint_id: int
    trails_paint_id: int
    goal_explosion_paint_id: int

    __match_args__ = (
        "car_paint_id",
        "decal_paint_id",
        "wheels_paint_id",
        "boost_paint_id",
        "antenna_paint_id",
        "hat_paint_id",
        "trails_paint_id",
        "goal_explosion_paint_id",
    )

    def __new__(
        cls,
        car_paint_id: int = 0,
        decal_paint_id: int = 0,
        wheels_paint_id: int = 0,
        boost_paint_id: int = 0,
        antenna_paint_id: int = 0,
        hat_paint_id: int = 0,
        trails_paint_id: int = 0,
        goal_explosion_paint_id: int = 0,
    ) -> LoadoutPaint: ...
    def __init__(
        self,
        car_paint_id: int = 0,
        decal_paint_id: int = 0,
        wheels_paint_id: int = 0,
        boost_paint_id: int = 0,
        antenna_paint_id: int = 0,
        hat_paint_id: int = 0,
        trails_paint_id: int = 0,
        goal_explosion_paint_id: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> LoadoutPaint:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RemoveRenderGroup:
    """
    A client message request removal of a RenderGroup.
    A client can only clear its own RenderGroups.
    """

    id: int

    __match_args__ = (
        "id",
    )

    def __new__(
        cls,
        id: int = 0,
    ) -> RemoveRenderGroup: ...
    def __init__(
        self,
        id: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> RemoveRenderGroup:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ScriptConfiguration:
    """
    A ScriptConfiguration defines a script of a match.
    """

    name: str
    """
    The name of the script.
    """
    root_dir: str
    """
    The root directory of the script and the working directory for the run command.
    """
    run_command: str
    """
    A console command that will start up the script.
    """
    script_id: int
    """
    The id of the script.
    This value is mostly used internally to keep track of participants in the match.
    """
    agent_id: str
    """
    A unique user-defined string that is used to connect clients to the right players/scripts.
    If a bot/script has a run command, RLBot will pass this agent id to the process using an environment variable, RLBOT_AGENT_ID.
    Upon connecting the process announces that it is responsible for this agent id and RLBot will pair the two.
    The recommended format for agent ids is "developername/botname".
    """

    __match_args__ = (
        "name",
        "root_dir",
        "run_command",
        "script_id",
        "agent_id",
    )

    def __new__(
        cls,
        name: str = "",
        root_dir: str = "",
        run_command: str = "",
        script_id: int = 0,
        agent_id: str = "",
    ) -> ScriptConfiguration: ...
    def __init__(
        self,
        name: str = "",
        root_dir: str = "",
        run_command: str = "",
        script_id: int = 0,
        agent_id: str = "",
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> ScriptConfiguration:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class SphereShape:
    """
    A sphere with diameter.
    Used for balls with a spherical shapes.
    """

    diameter: float

    __match_args__ = (
        "diameter",
    )

    def __new__(
        cls,
        diameter: float = 0.0,
    ) -> SphereShape: ...
    def __init__(
        self,
        diameter: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> SphereShape:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class StartCommand:
    """
    A client message to start a match using a path to a match config file.
    """

    config_path: str

    __match_args__ = (
        "config_path",
    )

    def __new__(
        cls,
        config_path: str = "",
    ) -> StartCommand: ...
    def __init__(
        self,
        config_path: str = "",
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> StartCommand:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class StopCommand:
    """
    A client message to stop a match and optionally the RLBot server too.
    """

    shutdown_server: bool

    __match_args__ = (
        "shutdown_server",
    )

    def __new__(
        cls,
        shutdown_server: bool = False,
    ) -> StopCommand: ...
    def __init__(
        self,
        shutdown_server: bool = False,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> StopCommand:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallAnchor:
    """
    A RenderAnchor attached to a ball.
    The local field allows for an offset in local coordinates taking the ball's orientation into account.
    """

    index: int
    """
    The index of the ball.
    """
    local: Vector3
    """
    An offset in local coordinates.
    x is forwards, y is left, and z is up.
    """

    __match_args__ = (
        "index",
        "local",
    )

    def __new__(
        cls,
        index: int = 0,
        local: Vector3 = Vector3(),
    ) -> BallAnchor: ...
    def __init__(
        self,
        index: int = 0,
        local: Vector3 = Vector3(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> BallAnchor:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallPrediction:
    """
    A prediction of a ball's trajectory, assuming no collision with cars.
    """

    slices: Sequence[PredictionSlice]
    """
    A list of predicted states of the ball at specific times in the future, assuming no collision with cars.
    The beginning of the list is now, and the end is 6 seconds into the future.
    The prediction is made at 120 Hz, resulting in 720 entries.
    """

    __match_args__ = (
        "slices",
    )

    def __new__(
        cls,
        slices: Sequence[PredictionSlice] = [],
    ) -> BallPrediction: ...
    def __init__(
        self,
        slices: Sequence[PredictionSlice] = [],
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> BallPrediction:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BoostPad:
    """
    Static information about a boost pad such as location and size.
    """

    location: Vector3
    """
    The location of the boost pad.
    """
    is_full_boost: bool
    """
    Whether the boost pad provides a full tank of boost.
    A big boost pad provides 100 boost and respawns in 10 seconds.
    A small boost pad provides 12 boost and respawns in 4 seconds.
    """

    __match_args__ = (
        "location",
        "is_full_boost",
    )

    def __new__(
        cls,
        location: Vector3 = Vector3(),
        is_full_boost: bool = False,
    ) -> BoostPad: ...
    def __init__(
        self,
        location: Vector3 = Vector3(),
        is_full_boost: bool = False,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> BoostPad:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class CarAnchor:
    """
    A RenderAnchor attached to a car.
    The local field allows for an offset in local coordinates taking the car's orientation into account.
    """

    index: int
    """
    The index of the car.
    """
    local: Vector3
    """
    An offset in local coordinates.
    x is forwards, y is left, and z is up.
    """

    __match_args__ = (
        "index",
        "local",
    )

    def __new__(
        cls,
        index: int = 0,
        local: Vector3 = Vector3(),
    ) -> CarAnchor: ...
    def __init__(
        self,
        index: int = 0,
        local: Vector3 = Vector3(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> CarAnchor:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ControllableTeamInfo:
    """
    Server message with information about the cars that the client can control.
    Sent to bot clients as a response to ConnectionSettings.
    There may be more than one car in case the bot is a hivemind.
    """

    team: int
    """
    The assigned team for this client.
    """
    controllables: Sequence[ControllableInfo]
    """
    The bots that RLBot will allow this client to control.
    """

    __match_args__ = (
        "team",
        "controllables",
    )

    def __new__(
        cls,
        team: int = 0,
        controllables: Sequence[ControllableInfo] = [],
    ) -> ControllableTeamInfo: ...
    def __init__(
        self,
        team: int = 0,
        controllables: Sequence[ControllableInfo] = [],
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> ControllableTeamInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class CustomBot:
    """
    A bot controlled by the RLBot framework.
    """

    name: str
    """
    Requested bot name. When match start, RLBot will ensure each bot has a unique name; bots with
    duplicate names will be renamed with a suffix like `(2)`
    """
    root_dir: str
    """
    The root directory of the bot and the working directory for the run command.
    """
    run_command: str
    """
    The console command that starts this bot.
    """
    loadout: PlayerLoadout | None
    """
    The loadout of the player.
    """
    agent_id: str
    """
    A unique user-defined string that is used to connect clients to the right players/scripts.
    If a bot/script has a run command, RLBot will pass this agent id to the process using an environment variable, RLBOT_AGENT_ID.
    Upon connecting the process announces that it is responsible for this agent id and RLBot will pair the two.
    The recommended format for agent ids is "developer_name/bot_name".
    """
    hivemind: bool
    """
    Whether this player is part of a hivemind bot where all players/cars are controlled by the same process.
    Players in the hivemind must have the same name, team, run command, and agent id.
    """

    __match_args__ = (
        "name",
        "root_dir",
        "run_command",
        "loadout",
        "agent_id",
        "hivemind",
    )

    def __new__(
        cls,
        name: str = "",
        root_dir: str = "",
        run_command: str = "",
        loadout: PlayerLoadout | None = None,
        agent_id: str = "",
        hivemind: bool = False,
    ) -> CustomBot: ...
    def __init__(
        self,
        name: str = "",
        root_dir: str = "",
        run_command: str = "",
        loadout: PlayerLoadout | None = None,
        agent_id: str = "",
        hivemind: bool = False,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> CustomBot:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DesiredBallState:
    """
    A ball state with nullable components.
    Used for game state setting to define which part of a ball's state should change.
    """

    physics: DesiredPhysics

    __match_args__ = (
        "physics",
    )

    def __new__(
        cls,
        physics: DesiredPhysics = DesiredPhysics(),
    ) -> DesiredBallState: ...
    def __init__(
        self,
        physics: DesiredPhysics = DesiredPhysics(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> DesiredBallState:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DesiredMatchInfo:
    """
    Match info with nullable components.
    Used for game state setting to define which part of the match info should change.
    """

    world_gravity_z: float | None
    """
    The strength of gravity. Default is usually -650 depending on mutators.
    To set gravity to 0, use 0.0000001 instead, as 0 will set gravity back to the default.
    """
    game_speed: float | None
    """
    The game speed. Default is 1.0.
    """

    __match_args__ = (
        "world_gravity_z",
        "game_speed",
    )

    def __new__(
        cls,
        world_gravity_z: float | None = None,
        game_speed: float | None = None,
    ) -> DesiredMatchInfo: ...
    def __init__(
        self,
        world_gravity_z: float | None = None,
        game_speed: float | None = None,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> DesiredMatchInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GoalInfo:
    """
    Static information about a goal on the field such as dimensions and location.
    More values can be found on https://wiki.rlbot.org/botmaking/useful-game-values/
    """

    team_num: int
    """
    The index of the team that this goal belongs to.
    """
    location: Vector3
    """
    The center location of the goal.
    """
    direction: Vector3
    """
    The unit direction point away from the opening of the goal.
    """
    width: float
    """
    The width of the goal. 1785 uu wide on a standard field.
    """
    height: float
    """
    The height of the goal. 643 uu tall on a standard field.
    """

    __match_args__ = (
        "team_num",
        "location",
        "direction",
        "width",
        "height",
    )

    def __new__(
        cls,
        team_num: int = 0,
        location: Vector3 = Vector3(),
        direction: Vector3 = Vector3(),
        width: float = 0.0,
        height: float = 0.0,
    ) -> GoalInfo: ...
    def __init__(
        self,
        team_num: int = 0,
        location: Vector3 = Vector3(),
        direction: Vector3 = Vector3(),
        width: float = 0.0,
        height: float = 0.0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> GoalInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MatchInfo:
    """
    Information about the current match such as time and gravity.
    """

    seconds_elapsed: float
    """
    How many seconds have elapsed since the first game packet of the match.
    This value ticks up even during kickoffs, replays, pause, etc.
    """
    game_time_remaining: float
    """
    Seconds remaining of the match.
    This value ticks up instead of down during overtime or when the game duration mutator is set to Unlimited.
    I.e. it matches the in-game timer at the top.
    """
    is_overtime: bool
    """
    True if the game is in overtime.
    """
    is_unlimited_time: bool
    """
    True if the game duration is set to Unlimited.
    """
    match_phase: MatchPhase
    """
    The current phase of the match, i.e. kickoff, replay, active, etc.
    """
    world_gravity_z: float
    """
    The current strength of gravity. Default is -650.
    """
    game_speed: float
    """
    Game speed multiplier. Regular game speed is 1.0.
    """
    last_spectated: int
    """
    Index of the player who was most recently a spectated by the host.
    """
    frame_num: int
    """
    Tracks the number of physics frames the game has computed.
    May increase by more than one across consecutive packets.
    Data type will roll over after 414 days at 120Hz.
    """

    __match_args__ = (
        "seconds_elapsed",
        "game_time_remaining",
        "is_overtime",
        "is_unlimited_time",
        "match_phase",
        "world_gravity_z",
        "game_speed",
        "last_spectated",
        "frame_num",
    )

    def __new__(
        cls,
        seconds_elapsed: float = 0.0,
        game_time_remaining: float = 0.0,
        is_overtime: bool = False,
        is_unlimited_time: bool = False,
        match_phase: MatchPhase = MatchPhase(),
        world_gravity_z: float = 0.0,
        game_speed: float = 0.0,
        last_spectated: int = 0,
        frame_num: int = 0,
    ) -> MatchInfo: ...
    def __init__(
        self,
        seconds_elapsed: float = 0.0,
        game_time_remaining: float = 0.0,
        is_overtime: bool = False,
        is_unlimited_time: bool = False,
        match_phase: MatchPhase = MatchPhase(),
        world_gravity_z: float = 0.0,
        game_speed: float = 0.0,
        last_spectated: int = 0,
        frame_num: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> MatchInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PlayerConfiguration:
    """
    A PlayerConfiguration defines a player of a match.
    """

    variety: CustomBot | Human | PsyonixBot
    """
    The type of the player, i.e. human, Psyonix bot, or a custom bot.
    """
    team: int
    """
    The team of the player. Blue is 0, orange is 1.
    """
    player_id: int
    """
    The value will be set by RLBot and is always overriden.
    This value is mostly used internally to keep track of participants in the match.
    The player id can be used to find the corresponding player in the GamePacket.
    """

    __match_args__ = (
        "variety",
        "team",
        "player_id",
    )

    def __new__(
        cls,
        variety: CustomBot | Human | PsyonixBot = CustomBot(),
        team: int = 0,
        player_id: int = 0,
    ) -> PlayerConfiguration: ...
    def __init__(
        self,
        variety: CustomBot | Human | PsyonixBot = CustomBot(),
        team: int = 0,
        player_id: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PlayerConfiguration:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PlayerInput:
    """
    A player index and the controller state of that player.
    Used to indicate what the player is doing this tick.
    """

    player_index: int
    controller_state: ControllerState

    __match_args__ = (
        "player_index",
        "controller_state",
    )

    def __new__(
        cls,
        player_index: int = 0,
        controller_state: ControllerState = ControllerState(),
    ) -> PlayerInput: ...
    def __init__(
        self,
        player_index: int = 0,
        controller_state: ControllerState = ControllerState(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PlayerInput:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RenderGroup:
    """
    A group of RenderMessages that are drawn and cleared together.
    A RenderGroup will stay rendered until it is overriden or cleared.
    The group is identified by a unique id.
    A client can only clear its own RenderGroups.
    """

    render_messages: Sequence[RenderMessage]
    """
    The content of the RenderGroup.
    """
    id: int
    """
    The id of the RenderGroup.
    """

    __match_args__ = (
        "render_messages",
        "id",
    )

    def __new__(
        cls,
        render_messages: Sequence[RenderMessage] = [],
        id: int = 0,
    ) -> RenderGroup: ...
    def __init__(
        self,
        render_messages: Sequence[RenderMessage] = [],
        id: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> RenderGroup:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RenderMessage:
    """
    A RenderMessage, describing a piece of debug rendering.
    """

    variety: Line3D | PolyLine3D | Rect2D | Rect3D | String2D | String3D

    __match_args__ = (
        "variety",
    )

    def __new__(
        cls,
        variety: Line3D | PolyLine3D | Rect2D | Rect3D | String2D | String3D = Line3D(),
    ) -> RenderMessage: ...
    def __init__(
        self,
        variety: Line3D | PolyLine3D | Rect2D | Rect3D | String2D | String3D = Line3D(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> RenderMessage:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RotatorPartial:
    """
    A rotator describing a rotation with nullable pith, yaw, and roll.
    Used for game state setting to define which part of a rotator should change.
    If a component is null, then the component will keep its current value.
    Values are in radians.
    """

    pitch: float | None
    """
    In radians with range (-pi/2,+pi/2) where 0 is flat, +pi/2 is nose straight up, -pi/2 is nose straight down.
    """
    yaw: float | None
    """
    In radians with range [-pi,+pi) where 0 is towards positive x, rotating clockwise as increased (when seen from above).
    """
    roll: float | None
    """
    In radians with range (-pi,+pi) where 0 is upright, positive is tilted right, negative is tilted left.
    """

    __match_args__ = (
        "pitch",
        "yaw",
        "roll",
    )

    def __new__(
        cls,
        pitch: float | None = None,
        yaw: float | None = None,
        roll: float | None = None,
    ) -> RotatorPartial: ...
    def __init__(
        self,
        pitch: float | None = None,
        yaw: float | None = None,
        roll: float | None = None,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> RotatorPartial:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class SetLoadout:
    """
    A client message to change the loadout of a car.
    If sent before the ready message, this simply sets the loadout of the car.
    If sent after the ready message and if game state setting is enabled, this will respawn the car with the new loadout.
    Bots can only set the loadout of their own car(s).
    """

    index: int
    """
    The index of the car to change loadout off.
    """
    loadout: PlayerLoadout
    """
    The new loadout of the car.
    """

    __match_args__ = (
        "index",
        "loadout",
    )

    def __new__(
        cls,
        index: int = 0,
        loadout: PlayerLoadout = PlayerLoadout(),
    ) -> SetLoadout: ...
    def __init__(
        self,
        index: int = 0,
        loadout: PlayerLoadout = PlayerLoadout(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> SetLoadout:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Touch:
    """
    Information about a ball touch.
    """

    game_seconds: float
    """
    Seconds that had elapsed in the game when the touch occurred.
    """
    location: Vector3
    """
    The point of contact for the touch.
    """
    normal: Vector3
    """
    The direction of the touch as a unit vector pointing from the point of contact towards the center of the ball.
    """
    ball_index: int
    """
    The index of the ball that was touched (in case there are multiple balls).
    """

    __match_args__ = (
        "game_seconds",
        "location",
        "normal",
        "ball_index",
    )

    def __new__(
        cls,
        game_seconds: float = 0.0,
        location: Vector3 = Vector3(),
        normal: Vector3 = Vector3(),
        ball_index: int = 0,
    ) -> Touch: ...
    def __init__(
        self,
        game_seconds: float = 0.0,
        location: Vector3 = Vector3(),
        normal: Vector3 = Vector3(),
        ball_index: int = 0,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> Touch:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Vector3Partial:
    """
    A 3D vector where x, y, and z can be null.
    Used for game state setting to define which part of a vector should change.
    If a component is null, then the component will keep its current value.
    """

    x: float | None
    y: float | None
    z: float | None

    __match_args__ = (
        "x",
        "y",
        "z",
    )

    def __new__(
        cls,
        x: float | None = None,
        y: float | None = None,
        z: float | None = None,
    ) -> Vector3Partial: ...
    def __init__(
        self,
        x: float | None = None,
        y: float | None = None,
        z: float | None = None,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> Vector3Partial:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class BallInfo:
    """
    Information about a ball.
    """

    physics: Physics
    """
    The physical state of the ball.
    """
    shape: BoxShape | CylinderShape | SphereShape
    """
    The collision shape of the ball.
    """

    __match_args__ = (
        "physics",
        "shape",
    )

    def __new__(
        cls,
        physics: Physics = Physics(),
        shape: BoxShape | CylinderShape | SphereShape = BoxShape(),
    ) -> BallInfo: ...
    def __init__(
        self,
        physics: Physics = Physics(),
        shape: BoxShape | CylinderShape | SphereShape = BoxShape(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> BallInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DesiredCarState:
    """
    A car state with nullable components.
    Used for game state setting to define which part of a car's state should change.
    """

    physics: DesiredPhysics | None
    boost_amount: float | None

    __match_args__ = (
        "physics",
        "boost_amount",
    )

    def __new__(
        cls,
        physics: DesiredPhysics | None = None,
        boost_amount: float | None = None,
    ) -> DesiredCarState: ...
    def __init__(
        self,
        physics: DesiredPhysics | None = None,
        boost_amount: float | None = None,
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> DesiredCarState:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class FieldInfo:
    """
    Static information about the field.
    Sent to bots, scripts, etc. upon connecting.
    Dynamic information is found in the GamePacket.
    """

    boost_pads: Sequence[BoostPad]
    """
    Static information about boost pads on the field.
    The dynamic information is found in the GamePacket
    The boost pads are ordered by y-coordinate and then x-coordinate.
    """
    goals: Sequence[GoalInfo]
    """
    Information about the goals on the field.
    """

    __match_args__ = (
        "boost_pads",
        "goals",
    )

    def __new__(
        cls,
        boost_pads: Sequence[BoostPad] = [],
        goals: Sequence[GoalInfo] = [],
    ) -> FieldInfo: ...
    def __init__(
        self,
        boost_pads: Sequence[BoostPad] = [],
        goals: Sequence[GoalInfo] = [],
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> FieldInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Line3D:
    """
    A RenderMessage for a line in 3D space between two RenderAnchors.
    """

    start: RenderAnchor
    end: RenderAnchor
    color: Color

    __match_args__ = (
        "start",
        "end",
        "color",
    )

    def __new__(
        cls,
        start: RenderAnchor = RenderAnchor(),
        end: RenderAnchor = RenderAnchor(),
        color: Color = Color(),
    ) -> Line3D: ...
    def __init__(
        self,
        start: RenderAnchor = RenderAnchor(),
        end: RenderAnchor = RenderAnchor(),
        color: Color = Color(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> Line3D:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PolyLine3D:
    """
    A RenderMessage for a line in 3D space going through a series of points.
    """

    points: Sequence[Vector3]
    color: Color

    __match_args__ = (
        "points",
        "color",
    )

    def __new__(
        cls,
        points: Sequence[Vector3] = [],
        color: Color = Color(),
    ) -> PolyLine3D: ...
    def __init__(
        self,
        points: Sequence[Vector3] = [],
        color: Color = Color(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PolyLine3D:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PsyonixBot:
    """
    A Psyonix bot, e.g. All Star bot.
    """

    name: str
    """
    Requested bot name. When match start, RLBot will ensure each bot has a unique name; bots with
    duplicate names will be renamed with a suffix like `(2)`. For psyonix bots, a blank name will
    be replaced with one of the official names.
    """
    loadout: PlayerLoadout | None
    """
    The loadout of the player.
    """
    bot_skill: PsyonixSkill

    __match_args__ = (
        "name",
        "loadout",
        "bot_skill",
    )

    def __new__(
        cls,
        name: str = "",
        loadout: PlayerLoadout | None = None,
        bot_skill: PsyonixSkill = PsyonixSkill(),
    ) -> PsyonixBot: ...
    def __init__(
        self,
        name: str = "",
        loadout: PlayerLoadout | None = None,
        bot_skill: PsyonixSkill = PsyonixSkill(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PsyonixBot:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Rect2D:
    """
    A RenderMessage for a rectangle in 2D space.
    Note that the position and size is given in screen-space coordinates.
    """

    x: float
    """
    Screen-space x coordinate such that x=0 is left edge and x=1 is right edge of window.
    """
    y: float
    """
    Screen-space y coordinate such that y=0 is top edge and y=1 is bottom edge of window.
    """
    width: float
    """
    Screen-space size such that width=0.1 is 10% of window width.
    """
    height: float
    """
    Screen-space size such that height=0.1 is 10% of window height.
    """
    color: Color
    """
    Color of the rectangle.
    """
    h_align: TextHAlign
    """
    The horizontal alignment of the rectangle.
    """
    v_align: TextVAlign
    """
    The vertical alignment of the rectangle.
    """

    __match_args__ = (
        "x",
        "y",
        "width",
        "height",
        "color",
        "h_align",
        "v_align",
    )

    def __new__(
        cls,
        x: float = 0.0,
        y: float = 0.0,
        width: float = 0.0,
        height: float = 0.0,
        color: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> Rect2D: ...
    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        width: float = 0.0,
        height: float = 0.0,
        color: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> Rect2D:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class String2D:
    """
    A RenderMessage for text in 2D space.
    Note that the position is given in screen-space coordinates.
    """

    text: str
    """
    The text to be displayed.
    """
    x: float
    """
    Screen-space x coordinate such that x=0 is left edge and x=1 is right edge of window.
    """
    y: float
    """
    Screen-space y coordinate such that y=0 is top edge and y=1 is bottom edge of window.
    """
    scale: float
    """
    Scale of the text.
    When scale is 1, the characters are 20 pixels tall and 10 pixels wide.
    """
    foreground: Color
    """
    The color of the text.
    """
    background: Color
    """
    The color of the background for the text.
    """
    h_align: TextHAlign
    """
    The horizontal alignment of the text.
    """
    v_align: TextVAlign
    """
    The vertical alignment of the text.
    """

    __match_args__ = (
        "text",
        "x",
        "y",
        "scale",
        "foreground",
        "background",
        "h_align",
        "v_align",
    )

    def __new__(
        cls,
        text: str = "",
        x: float = 0.0,
        y: float = 0.0,
        scale: float = 0.0,
        foreground: Color = Color(),
        background: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> String2D: ...
    def __init__(
        self,
        text: str = "",
        x: float = 0.0,
        y: float = 0.0,
        scale: float = 0.0,
        foreground: Color = Color(),
        background: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> String2D:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class DesiredGameState:
    """
    A game state with nullable subcomponents.
    Used for game state setting to define which part of the game should change.
    Values not set will not be updated.
    """

    ball_states: Sequence[DesiredBallState]
    """
    A list of desired ball states.
    """
    car_states: Sequence[DesiredCarState]
    """
    A list of desired car states.
    """
    match_info: DesiredMatchInfo | None
    """
    The desired game info.
    """
    console_commands: Sequence[ConsoleCommand]
    """
    A list of console commands to execute.
    See https://wiki.rlbot.org/framework/console-commands/ for a list of known commands.
    """

    __match_args__ = (
        "ball_states",
        "car_states",
        "match_info",
        "console_commands",
    )

    def __new__(
        cls,
        ball_states: Sequence[DesiredBallState] = [],
        car_states: Sequence[DesiredCarState] = [],
        match_info: DesiredMatchInfo | None = None,
        console_commands: Sequence[ConsoleCommand] = [],
    ) -> DesiredGameState: ...
    def __init__(
        self,
        ball_states: Sequence[DesiredBallState] = [],
        car_states: Sequence[DesiredCarState] = [],
        match_info: DesiredMatchInfo | None = None,
        console_commands: Sequence[ConsoleCommand] = [],
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> DesiredGameState:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Rect3D:
    """
    A RenderMessage for a rectangle in 3D space.
    Note that the size is given in screen-space sizes.
    """

    anchor: RenderAnchor
    """
    The position of the rectangle.
    """
    width: float
    """
    Screen-space size such that width=0.1 is 10% of window width.
    """
    height: float
    """
    Screen-space size such that height=0.1 is 10% of window height.
    """
    color: Color
    """
    The color of the rectangle.
    """
    h_align: TextHAlign
    """
    The horizontal alignment of the anchor in the rectangle.
    """
    v_align: TextVAlign
    """
    The vertical alignment of the anchor in the rectangle.
    """

    __match_args__ = (
        "anchor",
        "width",
        "height",
        "color",
        "h_align",
        "v_align",
    )

    def __new__(
        cls,
        anchor: RenderAnchor = RenderAnchor(),
        width: float = 0.0,
        height: float = 0.0,
        color: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> Rect3D: ...
    def __init__(
        self,
        anchor: RenderAnchor = RenderAnchor(),
        width: float = 0.0,
        height: float = 0.0,
        color: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> Rect3D:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class String3D:
    """
    A RenderMessage for text in 3D space.
    """

    text: str
    """
    The text to be displayed.
    """
    anchor: RenderAnchor
    """
    The position of the text.
    """
    scale: float
    """
    The scale of the text.
    When scale is 1, the characters are 20 pixels tall and 10 pixels wide.
    """
    foreground: Color
    """
    The color of the text.
    """
    background: Color
    """
    The color of the background for the text.
    """
    h_align: TextHAlign
    """
    The horizontal alignment of the text.
    """
    v_align: TextVAlign
    """
    The vertical alignment of the text.
    """

    __match_args__ = (
        "text",
        "anchor",
        "scale",
        "foreground",
        "background",
        "h_align",
        "v_align",
    )

    def __new__(
        cls,
        text: str = "",
        anchor: RenderAnchor = RenderAnchor(),
        scale: float = 0.0,
        foreground: Color = Color(),
        background: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> String3D: ...
    def __init__(
        self,
        text: str = "",
        anchor: RenderAnchor = RenderAnchor(),
        scale: float = 0.0,
        foreground: Color = Color(),
        background: Color = Color(),
        h_align: TextHAlign = TextHAlign(),
        v_align: TextVAlign = TextVAlign(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> String3D:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GamePacket:
    """
    A packet of data from the game.
    Is is sent every tick to bots, scripts, etc.
    Static data is found in the FieldInfo.
    """

    players: Sequence[PlayerInfo]
    """
    The current state of all players and their cars.
    """
    boost_pads: Sequence[BoostPadState]
    """
    The current state of all boost pads.
    The static information about boost pads are found in the FieldInfo.
    The boost pads are ordered by y-coordinate and then x-coordinate.
    """
    balls: Sequence[BallInfo]
    """
    The current state of all balls.
    """
    match_info: MatchInfo
    """
    The current state of the match such as timers and gravity.
    """
    teams: Sequence[TeamInfo]
    """
    The current state of teams, i.e. the team scores.
    """

    __match_args__ = (
        "players",
        "boost_pads",
        "balls",
        "match_info",
        "teams",
    )

    def __new__(
        cls,
        players: Sequence[PlayerInfo] = [],
        boost_pads: Sequence[BoostPadState] = [],
        balls: Sequence[BallInfo] = [],
        match_info: MatchInfo = MatchInfo(),
        teams: Sequence[TeamInfo] = [],
    ) -> GamePacket: ...
    def __init__(
        self,
        players: Sequence[PlayerInfo] = [],
        boost_pads: Sequence[BoostPadState] = [],
        balls: Sequence[BallInfo] = [],
        match_info: MatchInfo = MatchInfo(),
        teams: Sequence[TeamInfo] = [],
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> GamePacket:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class PlayerInfo:
    """
    A collection of information about a player and their car.
    """

    physics: Physics
    """
    The physical state of the player's car.
    """
    score_info: ScoreInfo
    """
    The various scores of this player, e.g. those on the leaderboard.
    """
    hitbox: BoxShape
    """
    The hitbox of the player's car.
    Note that the hitbox is not centered at the cars location.
    See the hitbox offset.
    """
    hitbox_offset: Vector3
    """
    The center of the hitbox in local coordinates.
    """
    latest_touch: Touch | None
    """
    Information about the latest touch with a ball.
    Is null if the player has yet to touch the ball.
    """
    air_state: AirState
    """
    Whether the player's car is on the ground or in the air, and what jump/dodging forces currently affects the car.
    """
    dodge_timeout: float
    """
    How long until the player cannot dodge/double jump anymore.
    The value is -1 while on ground or when airborne for too long after jumping.
    A dodge/double jump is possible for 1.25 seconds after the first jump plus
    up to an additional 0.2 seconds depending how long the jump button was pressed for the first jump.
    Note that falling off a surface instead of jumping, does not activate the dodge timeout, and making a dodge/double jump possible indefinitely.
    This is commonly known as a flip reset.
    The car is holding a flip reset if `air_state == InAir && !has_jumped && !has_double_jumped && !has_dodged`.
    """
    demolished_timeout: float
    """
    How long until the player is not demolished anymore.
    The value is -1 if while not demolished.
    """
    is_supersonic: bool
    """
    Whether the player's car is moving at supersonic speed and can demolish.
    """
    is_bot: bool
    """
    Whether the player is a bot or a human.
    """
    name: str
    """
    The name of the player as it appears in game, i.e. possibly appended with "(2)".
    The original name can be found in the match configuration.
    """
    team: int
    """
    The team of the player.
    """
    boost: float
    """
    The current boost, from 0 to 100.
    """
    player_id: int
    """
    The id of the player.
    This value is mostly used internally to keep track of participants in the match.
    The id can be used to find the corresponding PlayerConfiguration in the MatchConfiguration.
    """
    accolades: Sequence[str]
    """
    Events from the latest tick involving this player. Possible values include:
    Win, Loss, TimePlayed;
    Shot, Assist, Center, Clear, PoolShot;
    Goal, AerialGoal, BicycleGoal, BulletGoal, BackwardsGoal, LongGoal, OvertimeGoal, TurtleGoal;
    AerialHit, BicycleHit, BulletHit, JuggleHit, FirstTouch, BallHit;
    Save, EpicSave, FreezeSave;
    HatTrick, Savior, Playmaker, MVP;
    FastestGoal, SlowestGoal, FurthestGoal, OwnGoal;
    MostBallTouches, FewestBallTouches, MostBoostPickups, FewestBoostPickups, BoostPickups;
    CarTouches, Demolition, Demolish;
    LowFive, HighFive;
    Note that the list clears every tick.
    """
    last_input: ControllerState
    """
    The last controller input from this player.
    """
    has_jumped: bool
    """
    True if the player has jumped into the air.
    See dodge_timeout to know if a dodge/double jump is temporarily available.
    Note that falling off a surface instead of jumping, does not activate the dodge timeout, and making a dodge/double jump possible indefinitely.
    This is commonly known as a flip reset.
    The car is holding a flip reset if `air_state == InAir && !has_jumped && !has_double_jumped && !has_dodged`.
    """
    has_double_jumped: bool
    """
    True if the player has doubled jumped since it left the ground. False while on the ground.
    """
    has_dodged: bool
    """
    True if the player has dodged since it left the ground. False while the ground.
    """
    dodge_elapsed: float
    """
    The time in seconds since the last dodge was initiated.
    Resets to 0 when the player lands on the ground.
    """
    dodge_dir: Vector2
    """
    The unit direction of the latest dodge.
    The value will be (0,0) if it was a stall.
    """

    __match_args__ = (
        "physics",
        "score_info",
        "hitbox",
        "hitbox_offset",
        "latest_touch",
        "air_state",
        "dodge_timeout",
        "demolished_timeout",
        "is_supersonic",
        "is_bot",
        "name",
        "team",
        "boost",
        "player_id",
        "accolades",
        "last_input",
        "has_jumped",
        "has_double_jumped",
        "has_dodged",
        "dodge_elapsed",
        "dodge_dir",
    )

    def __new__(
        cls,
        physics: Physics = Physics(),
        score_info: ScoreInfo = ScoreInfo(),
        hitbox: BoxShape = BoxShape(),
        hitbox_offset: Vector3 = Vector3(),
        latest_touch: Touch | None = None,
        air_state: AirState = AirState(),
        dodge_timeout: float = 0.0,
        demolished_timeout: float = 0.0,
        is_supersonic: bool = False,
        is_bot: bool = False,
        name: str = "",
        team: int = 0,
        boost: float = 0.0,
        player_id: int = 0,
        accolades: Sequence[str] = [],
        last_input: ControllerState = ControllerState(),
        has_jumped: bool = False,
        has_double_jumped: bool = False,
        has_dodged: bool = False,
        dodge_elapsed: float = 0.0,
        dodge_dir: Vector2 = Vector2(),
    ) -> PlayerInfo: ...
    def __init__(
        self,
        physics: Physics = Physics(),
        score_info: ScoreInfo = ScoreInfo(),
        hitbox: BoxShape = BoxShape(),
        hitbox_offset: Vector3 = Vector3(),
        latest_touch: Touch | None = None,
        air_state: AirState = AirState(),
        dodge_timeout: float = 0.0,
        demolished_timeout: float = 0.0,
        is_supersonic: bool = False,
        is_bot: bool = False,
        name: str = "",
        team: int = 0,
        boost: float = 0.0,
        player_id: int = 0,
        accolades: Sequence[str] = [],
        last_input: ControllerState = ControllerState(),
        has_jumped: bool = False,
        has_double_jumped: bool = False,
        has_dodged: bool = False,
        dodge_elapsed: float = 0.0,
        dodge_dir: Vector2 = Vector2(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> PlayerInfo:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class MutatorSettings:
    """
    All mutators options.
    """

    match_length: MatchLengthMutator
    """
    Duration of the match.
    """
    max_score: MaxScoreMutator
    """
    Max score of match. If this score is reached, the team immediately wins.
    """
    multi_ball: MultiBallMutator
    """
    The number of balls.
    """
    overtime: OvertimeMutator
    """
    The overtime rules and tiebreaker.
    """
    series_length: SeriesLengthMutator
    """
    The series length.
    """
    game_speed: GameSpeedMutator
    """
    A game speed multiplier.
    """
    ball_max_speed: BallMaxSpeedMutator
    """
    Ball max speed.
    """
    ball_type: BallTypeMutator
    """
    Ball type and shape.
    """
    ball_weight: BallWeightMutator
    """
    Ball weight and how much is curves.
    """
    ball_size: BallSizeMutator
    """
    Ball size.
    """
    ball_bounciness: BallBouncinessMutator
    """
    Ball bounciness.
    """
    boost_amount: BoostAmountMutator
    """
    Boost amount/recharge.
    """
    rumble: RumbleMutator
    """
    Rumble item rules.
    """
    boost_strength: BoostStrengthMutator
    """
    Boost strength multiplier.
    """
    gravity: GravityMutator
    """
    Strength of gravity.
    """
    demolish: DemolishMutator
    """
    Demolition conditions.
    """
    respawn_time: RespawnTimeMutator
    """
    Demolition respawn time.
    """
    max_time: MaxTimeMutator
    """
    Max real-time duration of match including kickoff, replays, and more.
    If the score is tied upon time-out, the number of shots determine the winner.
    """
    game_event: GameEventMutator
    """
    Additional game behaviour for custom modes.
    """
    audio: AudioMutator
    """
    Additional audio options for custom modes.
    """
    ball_gravity: BallGravityMutator
    """
    Ball gravity.
    """
    territory: TerritoryMutator
    """
    Territory mutator.
    """
    stale_ball: StaleBallMutator
    """
    Stale ball mutator.
    """
    jump: JumpMutator
    """
    Jumps mutator.
    """
    dodge_timer: DodgeTimerMutator
    """
    Dodge timer mutator.
    """
    possession_score: PossessionScoreMutator
    """
    Possession score mutator.
    """
    demolish_score: DemolishScoreMutator
    """
    Demolish score mutator.
    """
    normal_goal_score: NormalGoalScoreMutator
    """
    Normal goal score mutator.
    """
    aerial_goal_score: AerialGoalScoreMutator
    """
    Aerial goal score mutator.
    """
    assist_goal_score: AssistGoalScoreMutator
    """
    Assist goal score mutator.
    """
    input_restriction: InputRestrictionMutator
    """
    Player input restriction mutator.
    """
    scoring_rule: ScoringRuleMutator
    """
    Additional rules about scoring (ball-goal interaction).
    """

    __match_args__ = (
        "match_length",
        "max_score",
        "multi_ball",
        "overtime",
        "series_length",
        "game_speed",
        "ball_max_speed",
        "ball_type",
        "ball_weight",
        "ball_size",
        "ball_bounciness",
        "boost_amount",
        "rumble",
        "boost_strength",
        "gravity",
        "demolish",
        "respawn_time",
        "max_time",
        "game_event",
        "audio",
        "ball_gravity",
        "territory",
        "stale_ball",
        "jump",
        "dodge_timer",
        "possession_score",
        "demolish_score",
        "normal_goal_score",
        "aerial_goal_score",
        "assist_goal_score",
        "input_restriction",
        "scoring_rule",
    )

    def __new__(
        cls,
        match_length: MatchLengthMutator = MatchLengthMutator(),
        max_score: MaxScoreMutator = MaxScoreMutator(),
        multi_ball: MultiBallMutator = MultiBallMutator(),
        overtime: OvertimeMutator = OvertimeMutator(),
        series_length: SeriesLengthMutator = SeriesLengthMutator(),
        game_speed: GameSpeedMutator = GameSpeedMutator(),
        ball_max_speed: BallMaxSpeedMutator = BallMaxSpeedMutator(),
        ball_type: BallTypeMutator = BallTypeMutator(),
        ball_weight: BallWeightMutator = BallWeightMutator(),
        ball_size: BallSizeMutator = BallSizeMutator(),
        ball_bounciness: BallBouncinessMutator = BallBouncinessMutator(),
        boost_amount: BoostAmountMutator = BoostAmountMutator(),
        rumble: RumbleMutator = RumbleMutator(),
        boost_strength: BoostStrengthMutator = BoostStrengthMutator(),
        gravity: GravityMutator = GravityMutator(),
        demolish: DemolishMutator = DemolishMutator(),
        respawn_time: RespawnTimeMutator = RespawnTimeMutator(),
        max_time: MaxTimeMutator = MaxTimeMutator(),
        game_event: GameEventMutator = GameEventMutator(),
        audio: AudioMutator = AudioMutator(),
        ball_gravity: BallGravityMutator = BallGravityMutator(),
        territory: TerritoryMutator = TerritoryMutator(),
        stale_ball: StaleBallMutator = StaleBallMutator(),
        jump: JumpMutator = JumpMutator(),
        dodge_timer: DodgeTimerMutator = DodgeTimerMutator(),
        possession_score: PossessionScoreMutator = PossessionScoreMutator(),
        demolish_score: DemolishScoreMutator = DemolishScoreMutator(),
        normal_goal_score: NormalGoalScoreMutator = NormalGoalScoreMutator(),
        aerial_goal_score: AerialGoalScoreMutator = AerialGoalScoreMutator(),
        assist_goal_score: AssistGoalScoreMutator = AssistGoalScoreMutator(),
        input_restriction: InputRestrictionMutator = InputRestrictionMutator(),
        scoring_rule: ScoringRuleMutator = ScoringRuleMutator(),
    ) -> MutatorSettings: ...
    def __init__(
        self,
        match_length: MatchLengthMutator = MatchLengthMutator(),
        max_score: MaxScoreMutator = MaxScoreMutator(),
        multi_ball: MultiBallMutator = MultiBallMutator(),
        overtime: OvertimeMutator = OvertimeMutator(),
        series_length: SeriesLengthMutator = SeriesLengthMutator(),
        game_speed: GameSpeedMutator = GameSpeedMutator(),
        ball_max_speed: BallMaxSpeedMutator = BallMaxSpeedMutator(),
        ball_type: BallTypeMutator = BallTypeMutator(),
        ball_weight: BallWeightMutator = BallWeightMutator(),
        ball_size: BallSizeMutator = BallSizeMutator(),
        ball_bounciness: BallBouncinessMutator = BallBouncinessMutator(),
        boost_amount: BoostAmountMutator = BoostAmountMutator(),
        rumble: RumbleMutator = RumbleMutator(),
        boost_strength: BoostStrengthMutator = BoostStrengthMutator(),
        gravity: GravityMutator = GravityMutator(),
        demolish: DemolishMutator = DemolishMutator(),
        respawn_time: RespawnTimeMutator = RespawnTimeMutator(),
        max_time: MaxTimeMutator = MaxTimeMutator(),
        game_event: GameEventMutator = GameEventMutator(),
        audio: AudioMutator = AudioMutator(),
        ball_gravity: BallGravityMutator = BallGravityMutator(),
        territory: TerritoryMutator = TerritoryMutator(),
        stale_ball: StaleBallMutator = StaleBallMutator(),
        jump: JumpMutator = JumpMutator(),
        dodge_timer: DodgeTimerMutator = DodgeTimerMutator(),
        possession_score: PossessionScoreMutator = PossessionScoreMutator(),
        demolish_score: DemolishScoreMutator = DemolishScoreMutator(),
        normal_goal_score: NormalGoalScoreMutator = NormalGoalScoreMutator(),
        aerial_goal_score: AerialGoalScoreMutator = AerialGoalScoreMutator(),
        assist_goal_score: AssistGoalScoreMutator = AssistGoalScoreMutator(),
        input_restriction: InputRestrictionMutator = InputRestrictionMutator(),
        scoring_rule: ScoringRuleMutator = ScoringRuleMutator(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> MutatorSettings:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class CorePacket:
    """
    Packet containing a CoreMessage
    """

    message: BallPrediction | ControllableTeamInfo | DisconnectSignal | FieldInfo | GamePacket | MatchComm | MatchConfiguration | PingRequest | PingResponse | RenderingStatus

    __match_args__ = (
        "message",
    )

    def __new__(
        cls,
        message: BallPrediction | ControllableTeamInfo | DisconnectSignal | FieldInfo | GamePacket | MatchComm | MatchConfiguration | PingRequest | PingResponse | RenderingStatus = BallPrediction(),
    ) -> CorePacket: ...
    def __init__(
        self,
        message: BallPrediction | ControllableTeamInfo | DisconnectSignal | FieldInfo | GamePacket | MatchComm | MatchConfiguration | PingRequest | PingResponse | RenderingStatus = BallPrediction(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> CorePacket:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class InterfacePacket:
    """
    Packet containing a InterfaceMessage
    """

    message: ConnectionSettings | DesiredGameState | DisconnectSignal | InitComplete | MatchComm | MatchConfiguration | PingRequest | PingResponse | PlayerInput | RemoveRenderGroup | RenderGroup | RenderingStatus | SetLoadout | StartCommand | StopCommand

    __match_args__ = (
        "message",
    )

    def __new__(
        cls,
        message: ConnectionSettings | DesiredGameState | DisconnectSignal | InitComplete | MatchComm | MatchConfiguration | PingRequest | PingResponse | PlayerInput | RemoveRenderGroup | RenderGroup | RenderingStatus | SetLoadout | StartCommand | StopCommand = ConnectionSettings(),
    ) -> InterfacePacket: ...
    def __init__(
        self,
        message: ConnectionSettings | DesiredGameState | DisconnectSignal | InitComplete | MatchComm | MatchConfiguration | PingRequest | PingResponse | PlayerInput | RemoveRenderGroup | RenderGroup | RenderingStatus | SetLoadout | StartCommand | StopCommand = ConnectionSettings(),
    ) -> None:
        """
        NOTE: All field initialization before `__init__`, inside of `__new__`.
        """
    def pack(self) -> bytes:
        """
        Serializes this instance into a byte array
        """

    @staticmethod
    def unpack(data: bytes) -> InterfacePacket:
        """
        Deserializes the data into a new instance

        :raises InvalidFlatbuffer: If the `data` is invalid for this type
        """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
