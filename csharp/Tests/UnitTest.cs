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
        var a = TreeNode.Make([6, 2, 13, 1, 4, 9, 15, null, null, null, null, null, null, 14]);
        Assert.AreEqual("[[2,2],[4,6],[15,-1]]", sol.ClosestNodes(a, [2, 5, 16]).Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([4, null, 9]);
        Assert.AreEqual("[[-1,4]]", sol.ClosestNodes(a, [3]).Print());
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