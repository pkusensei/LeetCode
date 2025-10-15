using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(true, new[] { 1, 1, 1, 1, 2, 2, 2, 2 }, 2)]
    public void TestMethod1(bool exp, int[] nums, int k)
    {
        Assert.AreEqual(exp, sol.CanPartitionKSubsets(nums, k));
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