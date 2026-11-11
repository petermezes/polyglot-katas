using Exercises.Ex07Options;
using Xunit;

namespace Exercises.Tests.Ex07Options;

public class ClientBuilderTests
{
    [Fact]
    public void Build_UsesDefaults()
    {
        var o = new ClientBuilder("https://api.example.no").Build();

        Assert.Equal("https://api.example.no", o.BaseUrl);
        Assert.Equal(TimeSpan.FromSeconds(30), o.Timeout);
        Assert.Equal(3, o.Retries);
        Assert.Equal("exercises/1.0", o.UserAgent);
        Assert.Empty(o.Headers);
    }

    [Fact]
    public void Build_AppliesFluentCalls()
    {
        var o = new ClientBuilder("https://api.example.no")
            .WithTimeout(TimeSpan.FromSeconds(5))
            .WithRetries(0)
            .WithHeader("Accept", "application/json")
            .WithHeader("X-Tunnel", "Løvstakk")
            .WithUserAgent("vegvokter-test")
            .Build();

        Assert.Equal(TimeSpan.FromSeconds(5), o.Timeout);
        Assert.Equal(0, o.Retries);
        Assert.Equal("vegvokter-test", o.UserAgent);
        Assert.Equal(2, o.Headers.Count);
        Assert.Equal("Løvstakk", o.Headers["X-Tunnel"]);
    }

    [Fact]
    public void InvalidValues_Throw()
    {
        Assert.Throws<ArgumentException>(() => new ClientBuilder("http://api.example.no").Build());
        Assert.Throws<ArgumentOutOfRangeException>(() => new ClientBuilder("https://x").WithTimeout(TimeSpan.Zero));
        Assert.Throws<ArgumentOutOfRangeException>(() => new ClientBuilder("https://x").WithRetries(11));
        Assert.Throws<ArgumentOutOfRangeException>(() => new ClientBuilder("https://x").WithRetries(-1));
    }

    [Fact]
    public void Build_SnapshotsHeaders()
    {
        var builder = new ClientBuilder("https://x").WithHeader("A", "1");
        var first = builder.Build();

        builder.WithHeader("B", "2");

        Assert.Single(first.Headers);
        Assert.Equal(2, builder.Build().Headers.Count);
    }

    [Fact]
    public void ObjectInitializer_AndWithExpression()
    {
        // Without the builder: object initializer + a required property.
        var o = new ClientOptions { BaseUrl = "https://x", Retries = 5 };

        var slower = o.Slower(2);

        Assert.Equal(TimeSpan.FromSeconds(60), slower.Timeout);
        Assert.Equal(5, slower.Retries);
        Assert.Equal(TimeSpan.FromSeconds(30), o.Timeout);
        Assert.NotEqual(o, slower);
        Assert.Equal(o, o with { });
    }
}
