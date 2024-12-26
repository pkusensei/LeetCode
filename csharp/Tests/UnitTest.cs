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
        var a = TreeNode.Make([2, 1, 4]);
        var b = TreeNode.Make([1, 0, 3]);
        Assert.AreEqual("[0,1,1,2,3,4]", sol.GetAllElements(a, b).Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, null, 8]);
        var b = TreeNode.Make([8, 1]);
        Assert.AreEqual("[1,1,8,8]", sol.GetAllElements(a, b).Print());
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