using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 2, 3, 5, 9 }, 2, 5)]
    [DataRow(new[] { 2, 7, 9, 3, 1 }, 2, 2)]
    public void TestMethod1(int[] nums, int k, int exp)
    {
        Assert.AreEqual(exp, sol.MinCapability(nums, k));
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