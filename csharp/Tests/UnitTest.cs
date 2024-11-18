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
        var a = TreeNode.Make([4, 1, 3, null, null, 2]);
        var b = sol.InsertIntoMaxTree(a, 5);
        Assert.AreEqual("[5,4,null,1,3,null,null,2]", b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([5, 2, 4, null, 1]);
        var b = sol.InsertIntoMaxTree(a, 3);
        Assert.AreEqual("[5,2,4,null,1,null,3]", b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([5, 2, 3, null, 1]);
        var b = sol.InsertIntoMaxTree(a, 4);
        Assert.AreEqual("[5,2,4,null,1,3]", b.ToString());
    }
}