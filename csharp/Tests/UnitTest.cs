using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 3, 5, 2, 7, 5 }, 1, 5, 2)]
    [DataRow(new[] { 1, 1, 1, 1 }, 1, 1, 10)]
    public void TestMethod1(int[] nums, int minK, int maxK, int exp)
    {
        Assert.AreEqual(exp, sol.CountSubarrays(nums, minK, maxK));
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