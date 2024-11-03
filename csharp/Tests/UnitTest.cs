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
        var a = TreeNode.Make([5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9]);
        var b = sol.IncreasingBST(a);
        var c = "[1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 1, 7]);
        var b = sol.IncreasingBST(a);
        var c = "[1,null,5,null,7]";
        Assert.AreEqual(c, b.ToString());
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