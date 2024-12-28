using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 1, 2, 1, 2, 6, 7, 5, 1 }, 2, "[0,3,5]")]
    [DataRow(new int[] { 1, 2, 1, 2, 1, 2, 1, 2, 1 }, 2, "[0,2,4]")]
    [DataRow(new int[] { 7, 13, 20, 19, 19, 2, 10, 1, 1, 19 }, 3, "[1,4,7]")]
    public void TestMethod1(int[] nums, int k, string exp)
    {
        Assert.AreEqual(exp, sol.MaxSumOfThreeSubarrays(nums, k).Print());
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