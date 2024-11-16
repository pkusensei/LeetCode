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
        var a = TreeNode.Make([0, 1, 2, 3, 4, 3, 4]);
        var b = sol.SmallestFromLeaf(a);
        Assert.AreEqual("dba", b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([25, 1, 3, 1, 3, 0, 2]);
        var b = sol.SmallestFromLeaf(a);
        Assert.AreEqual("adz", b);
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([2, 2, 1, null, 1, 0, null, 0]);
        var b = sol.SmallestFromLeaf(a);
        Assert.AreEqual("abc", b);
    }
}