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
        var arr = new[] { new[] { 1, 4, 3, 1, 3, 2 }, [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1] };
        Assert.AreEqual(4, sol.TrapRainWater(arr));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var arr = new[] { new[] { 3, 3, 3, 3, 3 }, [3, 2, 2, 2, 3], [3, 2, 1, 2, 3], [3, 2, 2, 2, 3], [3, 3, 3, 3, 3] };
        Assert.AreEqual(10, sol.TrapRainWater(arr));
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}