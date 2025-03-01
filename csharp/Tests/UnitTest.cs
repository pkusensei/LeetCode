using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 2, 2, 1, 2 }, 3)]
    [DataRow(new[] { 1, 2, 0 }, 3)]
    [DataRow(new[] { 3, 0, 1, 4, 1 }, 4)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.MaximumInvitations(nums));
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