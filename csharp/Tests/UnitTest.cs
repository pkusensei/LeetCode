using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, new[] { 1, 3 }, 6)]
    [DataRow(2, new[] { 1, 5, 10 }, 20)]
    [DataRow(0, new[] { 1, 2, 2 }, 5)]
    public void TestMethod1(int exp, int[] nums, int n)
    {
        Assert.AreEqual(exp, sol.MinPatches(nums, n));
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