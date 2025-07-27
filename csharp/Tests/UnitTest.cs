using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(167, new[] { 3, 1, 5, 8 })]
    [DataRow(10, new[] { 1, 5 })]
    public void TestMethod1(int exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.MaxCoins(nums));
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