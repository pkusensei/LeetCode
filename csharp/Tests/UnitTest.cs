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
        // var a = TreeNode.Make([3, 2, 1, 6, 0, 5]);
        var b = sol.WithStack([3, 2, 1, 6, 0, 5]);
        var c = "[6,3,5,null,2,0,null,null,1]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        // var a = TreeNode.Make([3, 2, 1]);
        var b = sol.WithStack([3, 2, 1]);
        var c = "[3,null,2,null,1]";
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