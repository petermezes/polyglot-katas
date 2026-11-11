using Exercises.Ex06States;
using Xunit;

namespace Exercises.Tests.Ex06States;

public class LaneTests
{
    [Theory]
    [InlineData(LaneState.Open, LaneAction.Restrict, LaneState.Restricted)]
    [InlineData(LaneState.Open, LaneAction.Close, LaneState.Closed)]
    [InlineData(LaneState.Restricted, LaneAction.Close, LaneState.Closed)]
    [InlineData(LaneState.Restricted, LaneAction.Reopen, LaneState.Open)]
    [InlineData(LaneState.Closed, LaneAction.Reopen, LaneState.Open)]
    [InlineData(LaneState.Closed, LaneAction.StartMaintenance, LaneState.Maintenance)]
    [InlineData(LaneState.Maintenance, LaneAction.EndMaintenance, LaneState.Closed)]
    public void Next_ValidTransitions(LaneState from, LaneAction action, LaneState expected)
    {
        Assert.Equal(expected, LaneMachine.Next(from, action));
    }

    [Theory]
    [InlineData(LaneState.Open, LaneAction.Reopen)]
    [InlineData(LaneState.Open, LaneAction.StartMaintenance)]
    [InlineData(LaneState.Restricted, LaneAction.StartMaintenance)]
    [InlineData(LaneState.Maintenance, LaneAction.Reopen)]
    [InlineData(LaneState.Closed, LaneAction.Close)]
    public void Next_InvalidTransitions_Throw(LaneState from, LaneAction action)
    {
        var ex = Assert.Throws<InvalidTransitionException>(() => LaneMachine.Next(from, action));

        Assert.Equal(from, ex.From);
        Assert.Equal(action, ex.Action);
    }

    [Fact]
    public void InvalidTransition_HasReadableMessage()
    {
        var ex = Assert.Throws<InvalidTransitionException>(() => LaneMachine.Next(LaneState.Maintenance, LaneAction.Reopen));

        Assert.Equal("maintenance + reopen: invalid transition", ex.Message);
    }

    [Fact]
    public void Lane_TracksStateAndHistory()
    {
        var lane = new Lane("Løvstakk lane 1");
        Assert.Equal(LaneState.Open, lane.State);

        lane.Apply(LaneAction.Restrict, LaneAction.Close, LaneAction.StartMaintenance, LaneAction.EndMaintenance);

        Assert.Equal(
            new[] { LaneState.Open, LaneState.Restricted, LaneState.Closed, LaneState.Maintenance, LaneState.Closed },
            lane.History);

        Assert.Throws<InvalidTransitionException>(() =>
            lane.Apply(LaneAction.Reopen, LaneAction.Restrict, LaneAction.Reopen, LaneAction.StartMaintenance));
        Assert.Equal(LaneState.Open, lane.State);
    }

    [Fact]
    public void Lane_HistoryCannotBeModifiedFromOutside()
    {
        var lane = new Lane("x");

        // IReadOnlyList cannot be cast back to List and modified.
        Assert.False(lane.History is List<LaneState>);
    }
}
