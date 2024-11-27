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
        var a = TreeNode.Make(new int[] { 1, 2, 3, 4, -99, -99, 7, 8, 9, -99, -99, 12, 13, -99, 14 });
        var b = sol.SufficientSubset(a, 1);
        Assert.AreEqual("[1,2,3,4,null,null,7,8,9,null,14]", b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 4, 8, 11, null, 17, 4, 7, 1, null, null, 5, 3]);
        var b = sol.SufficientSubset(a, 22);
        Assert.AreEqual("[5,4,8,11,null,17,4,7,null,null,null,5]", b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 2, -3, -5, null, 4, null]);
        var b = sol.SufficientSubset(a, -1);
        Assert.AreEqual("[1,null,-3,4]", b.ToString());
    }

    [TestMethod]
    public void TestMethod4()
    {
        var a = TreeNode.Make(new int[] { 10, 5, 10 });
        var b = sol.SufficientSubset(a, 21);
        Assert.IsTrue(b is null);
    }
}