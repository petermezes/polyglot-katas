// Exercise 06: state machine — enums, switch on tuples, encapsulation, params.
namespace Exercises.Ex06States;

public enum LaneState { Open, Restricted, Closed, Maintenance }

public enum LaneAction { Restrict, Close, Reopen, StartMaintenance, EndMaintenance }

public class InvalidTransitionException : InvalidOperationException
{
    public LaneState From { get; }
    public LaneAction Action { get; }

    public InvalidTransitionException(LaneState from, LaneAction action)
        : base($"{LaneMachine.Name(from)} + {LaneMachine.Name(action)}: invalid transition")
    {
        From = from;
        Action = action;
    }
}

public static class LaneMachine
{
    /// <summary>
    /// Open --Restrict--> Restricted, Open --Close--> Closed,
    /// Restricted --Close--> Closed, Restricted --Reopen--> Open,
    /// Closed --Reopen--> Open, Closed --StartMaintenance--> Maintenance,
    /// Maintenance --EndMaintenance--> Closed. Otherwise InvalidTransitionException.
    /// </summary>
    public static LaneState Next(LaneState state, LaneAction action) => (state, action) switch
    {
        (LaneState.Open, LaneAction.Restrict) => LaneState.Restricted,
        (LaneState.Open, LaneAction.Close) => LaneState.Closed,
        (LaneState.Restricted, LaneAction.Close) => LaneState.Closed,
        (LaneState.Restricted, LaneAction.Reopen) => LaneState.Open,
        (LaneState.Closed, LaneAction.Reopen) => LaneState.Open,
        (LaneState.Closed, LaneAction.StartMaintenance) => LaneState.Maintenance,
        (LaneState.Maintenance, LaneAction.EndMaintenance) => LaneState.Closed,
        _ => throw new InvalidTransitionException(state, action),
    };

    public static string Name(LaneState s) => s switch
    {
        LaneState.Open => "open",
        LaneState.Restricted => "restricted",
        LaneState.Closed => "closed",
        LaneState.Maintenance => "maintenance",
        _ => $"LaneState({(int)s})",
    };

    public static string Name(LaneAction a) => a switch
    {
        LaneAction.Restrict => "restrict",
        LaneAction.Close => "close",
        LaneAction.Reopen => "reopen",
        LaneAction.StartMaintenance => "start maintenance",
        LaneAction.EndMaintenance => "end maintenance",
        _ => $"LaneAction({(int)a})",
    };
}

public class Lane
{
    private readonly List<LaneState> _history = [LaneState.Open];

    public Lane(string name) => Name = name;

    public string Name { get; }

    public LaneState State { get; private set; } = LaneState.Open;

    /// <summary>Read-only — the history can't be changed from outside.</summary>
    public IReadOnlyList<LaneState> History => _history.AsReadOnly();

    /// <summary>Performs the actions in order; on the first invalid one it throws and the state stays as after the last valid one.</summary>
    public void Apply(params LaneAction[] actions)
    {
        foreach (var action in actions)
        {
            State = LaneMachine.Next(State, action);
            _history.Add(State);
        }
    }
}
