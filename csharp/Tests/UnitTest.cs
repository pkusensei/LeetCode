using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, new[] { 1 }, 1)]
    [DataRow(2, new[] { 17, 85, 93, -45, -21 }, 150)]
    public void TestMethod1(int exp, int[] nums, int k)
    {
        Assert.AreEqual(exp, sol.ShortestSubarray(nums, k));
        Assert.AreEqual(exp, sol.WithDeque(nums, k));
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