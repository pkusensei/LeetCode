using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = TreeNode.Make([5, 3, 6, 2, 4, null, 7]);
        var b = sol.DeleteNode(a, 3);
        var c = "[5,4,6,2,null,null,7]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 3, 6, 2, 4, null, 7]);
        var b = sol.DeleteNode(a, 0);
        var c = "[5,3,6,2,4,null,7]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([5, 3, 6, 2, 4, null, 7]);
        var b = sol.DeleteNode(a, 5);
        var c = "[6,3,7,2,4]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod4()
    {
        var a = TreeNode.Make([1,null,2]);
        var b = sol.DeleteNode(a, 1);
        var c = "[2]";
        Assert.AreEqual(c, b.ToString());
    }

}