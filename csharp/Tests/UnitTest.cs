using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, new[] { 1, 0, 1 }, 1, 1)]
    [DataRow(4, new[] { 1, 0, 3 }, 1, 2)]
    public void TestMethod1(int exp, int[] nums, int a, int b)
    {
        Assert.AreEqual(exp, sol.MaxSumTwoNoOverlap(nums, a, b));
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