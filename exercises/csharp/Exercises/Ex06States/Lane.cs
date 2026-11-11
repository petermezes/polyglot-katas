// Exercise 06: state machine — enums, switch on tuples, encapsulation, params.
namespace Exercises.Ex06States;

public enum LaneState { Open, Restricted, Closed, Maintenance }

public enum LaneAction { Restrict, Close, Reopen, StartMaintenance, EndMaintenance }

/// <summary>Message: "{Name(from)} + {Name(action)}: invalid transition", e.g. "maintenance + reopen: invalid transition".</summary>
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
    /// Tip: switch on a tuple: (state, action) switch { (LaneState.Open, LaneAction.Restrict) =&gt; ..., _ =&gt; throw ... }
    /// </summary>
    public static LaneState Next(LaneState state, LaneAction action) =>
        throw new NotImplementedException();

    /// <summary>"open", "restricted", "closed", "maintenance"; unknown e.g. "LaneState(42)".</summary>
    public static string Name(LaneState s) => throw new NotImplementedException();

    /// <summary>"restrict", "close", "reopen", "start maintenance", "end maintenance".</summary>
    public static string Name(LaneAction a) => throw new NotImplementedException();
}

public class Lane
{
    // TODO: a private history list (starts with the Open state)

    public Lane(string name) => Name = name;

    public string Name { get; }

    /// <summary>Tip: { get; private set; } — read-only from outside.</summary>
    public LaneState State => throw new NotImplementedException();

    /// <summary>Read-only history — return a wrapper that can't be cast to List (AsReadOnly).</summary>
    public IReadOnlyList<LaneState> History => throw new NotImplementedException();

    /// <summary>
    /// Performs the actions in order; on the first invalid one it throws and the state stays as after the last valid one.
    /// Each new state is appended to the history.
    /// </summary>
    public void Apply(params LaneAction[] actions) => throw new NotImplementedException();
}
