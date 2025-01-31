using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var grid = new[] { new[] { 1, 0 }, [0, 1] };
        Assert.AreEqual(3, sol.LargestIsland(grid));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var grid = new[] { new[] { 1, 1 }, [0, 1] };
        Assert.AreEqual(4, sol.LargestIsland(grid));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var grid = new[] { new[] { 1, 1 }, [1, 1] };
        Assert.AreEqual(4, sol.LargestIsland(grid));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}