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
        var a = TreeNode.Make([4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8]);
        var b = sol.ConvertBST(a);
        var c = "[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([0, null, 1]);
        var b = sol.ConvertBST(a);
        var c = "[1,null,1]";
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