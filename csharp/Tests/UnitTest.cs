using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, new[] { -2, 5, -1 }, -2, 2)]
    [DataRow(1, new[] { 0 }, 0, 0)]
    public void TestMethod1(int exp, int[] nums, int lower, int upper)
    {
        Assert.AreEqual(exp, sol.CountRangeSum(nums, lower, upper));
        Assert.AreEqual(exp, sol.WithMergeSort(nums, lower, upper));
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