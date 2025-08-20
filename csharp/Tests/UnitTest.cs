using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(true, new[] { 2, -1, 1, 2, 2 })]
    [DataRow(false, new[] { -1, -2, -3, -4, -5, 6 })]
    [DataRow(true, new[] { 1, -1, 5, 1, 4 })]
    public void TestMethod1(bool exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.CircularArrayLoop(nums));
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