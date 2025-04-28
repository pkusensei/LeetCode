using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 2, 1, 4, 3, 5 }, 10, 6)]
    [DataRow(new[] { 1, 1, 1 }, 5, 5)]
    public void TestMethod1(int[] nums, int k, int exp)
    {
        Assert.AreEqual(exp, sol.CountSubarrays(nums, k));
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