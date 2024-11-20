using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 1, 0, 1, 0, 1, 0, 1 }, 22)]
    [DataRow(new int[] { 0 }, 0)]
    public void TestMethod1(int[] nums, int c)
    {
        var a = TreeNode.Make(nums);
        var b = sol.SumRootToLeaf(a);
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }
}