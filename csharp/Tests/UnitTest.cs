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
        // var a = TreeNode.Make([4, 2, 7, 1, 3]);
        var b = sol.Search([-1, 0, 3, 5, 9, 12], target: 9);
        var c = 4;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        // var a = TreeNode.Make([40, 20, 60, 10, 30, 50, 70]);
        var b = sol.Search([-1, 0, 3, 5, 9, 12], target: 2);
        var c = -1;
        Assert.AreEqual(c, b);
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