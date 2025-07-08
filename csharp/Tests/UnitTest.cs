using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 5, 7, 7, 8, 8, 10 }, 8, new[] { 3, 4 })]
    public void TestMethod1(int[] nums, int target, int[] exp)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.SearchRange(nums, target)));
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