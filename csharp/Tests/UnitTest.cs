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
        var a = TreeNode.Make([5, 4, 9, 1, 10, null, 7]);
        Assert.AreEqual("[0,0,0,7,7,null,11]", sol.ReplaceValueInTree(a).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([3, 1, 2]);
        Assert.AreEqual("[0,0,0]", sol.ReplaceValueInTree(a).ToString());
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