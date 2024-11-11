using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, new[] { " /", "/ " })]
    [DataRow(1, new[] { " /", "  " })]
    [DataRow(5, new[] { "/\\", "\\/" })]
    public void TestMethod1(int a, string[] grid)
    {
        var b = sol.RegionsBySlashes(grid);
        Assert.AreEqual(a, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
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