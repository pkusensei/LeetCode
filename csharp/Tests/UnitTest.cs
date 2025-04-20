using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 1, 2 }, 5)]
    [DataRow(new[] { 10, 10, 10 }, 11)]
    [DataRow(new[] { 0, 0, 1, 1, 1 }, 6)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.NumRabbits(nums));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}