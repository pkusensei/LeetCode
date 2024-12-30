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
        var a = TreeNode.Make([1, 2, 3, 2, null, 2, 4]);
        Assert.AreEqual("[1,null,3,null,4]", sol.RemoveLeafNodes(a, 2).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 3, 3, 3, 2]);
        Assert.AreEqual("[1,3,null,null,2]", sol.RemoveLeafNodes(a, 3).ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 2, null, 2, null, 2]);
        Assert.AreEqual("[1]", sol.RemoveLeafNodes(a, 2).ToString());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}