using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 0 }, 3)]
    [DataRow(new[] { 3, 4, -1, 1 }, 2)]
    [DataRow(new[] { 7, 8, 9, 11, 12 }, 1)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.FirstMissingPositive([.. nums]));
        Assert.AreEqual(exp, sol.WithCycleSort(nums));
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