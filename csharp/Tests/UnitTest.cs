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
        var a = TreeNode.Make([3, 9, 20, null, null, 15, 7]);
        var b = sol.VerticalTraversal(a);
        Assert.AreEqual("[[9],[3,15],[20],[7]]", b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 3, 4, 5, 6, 7]);
        var b = sol.VerticalTraversal(a);
        Assert.AreEqual("[[4],[2],[1,5,6],[3],[7]]", b.Print());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 2, 3, 4, 6, 5, 7]);
        var b = sol.VerticalTraversal(a);
        Assert.AreEqual("[[4],[2],[1,5,6],[3],[7]]", b.Print());
    }
}