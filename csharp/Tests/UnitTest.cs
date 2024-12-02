using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 2, 7, 9, 4, 4 }, 10)]
    [DataRow(new int[] { 1, 2, 3, 4, 5, 100 }, 104)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.StoneGameII(nums));
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