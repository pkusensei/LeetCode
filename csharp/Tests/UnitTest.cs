using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 2, 1, 2, 4, 2, 2 }, new[] { 5, 2, 6, 2, 3, 2 }, 2)]
    [DataRow(new[] { 3, 5, 1, 2, 3 }, new[] { 3, 6, 3, 3, 4 }, -1)]
    [DataRow(new[] { 1, 2, 1, 1, 1, 2, 2, 2 }, new[] { 2, 1, 2, 2, 2, 2, 2, 2 }, 1)]
    public void TestMethod1(int[] top, int[] bottom, int exp)
    {
        Assert.AreEqual(exp, sol.MinDominoRotations(top, bottom));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}