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
        var a = TreeNode.Make(new int[] { 1, 2, 3, 4, 5, 6, 7 });
        var b = sol.DelNodes(a, [3, 5]);
        Assert.AreEqual("[[1,2,null,4],[6],[7]]", b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 4, null, 3]);
        var b = sol.DelNodes(a, [3]);
        Assert.AreEqual("[[1,2,4]]", b.Print());
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