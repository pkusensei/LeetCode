using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 0, 1, 7, 4, 4, 5 }, 3, 6, 6)]
    [DataRow(new[] { 1, 7, 9, 2, 5 }, 11, 11, 1)]
    public void TestMethod1(int[] nums, int lower, int upper, int exp)
    {
        Assert.AreEqual(exp, sol.CountFairPairs(nums, lower, upper));
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