using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, new[] { 1, 3, 5, 4, 7 })]
    [DataRow(5, new[] { 2, 2, 2, 2, 2 })]
    public void TestMethod1(int exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.FindNumberOfLIS(nums));
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