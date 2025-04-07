using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 2, 2 }, 2)]
    [DataRow(new[] { 2, 1, 3, 1, 1, 1, 7, 1, 2, 1 }, 4)]
    [DataRow(new[] { 3, 3, 3, 3, 7, 2, 2 }, -1)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.MinimumIndex(nums));
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