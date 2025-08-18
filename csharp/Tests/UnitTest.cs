using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(true, new[] { 5, 5, 5, 1 })]
    [DataRow(true, new[] { 4, 1, 8, 7 })]
    [DataRow(false, new[] { 1, 2, 1, 2 })]
    public void TestMethod1(bool exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.JudgePoint24(nums));
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