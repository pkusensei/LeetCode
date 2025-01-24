using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = TreeNode.Make([1, 10, 4, 3, null, 7, 9, 12, 8, 6, null, null, 2]);
        Assert.IsTrue(sol.IsEvenOddTree(a));
    }

    [TestMethod]
    [DataRow(new[] { 5, 4, 2, 3, 3, 7 }, false)]
    [DataRow(new[] { 5, 9, 1, 3, 5, 7 }, false)]
    public void TestMethod2(int[] nums, bool exp)
    {
        var a = TreeNode.MakeInt(nums);
        Assert.IsFalse(sol.IsEvenOddTree(a));
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