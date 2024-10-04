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
        var a = TreeNode.Make([4, 2, 6, 3, 1, 5]);
        var b = sol.AddOneRow(a, 1, 2);
        var c = "[4,1,1,2,null,null,6,3,1,5]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([4, 2, null, 3, 1]);
        var b = sol.AddOneRow(a, 1, 3);
        var c = "[4,2,null,1,1,3,null,null,1]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([1, 2, 3, 4]);
        var b = sol.AddOneRow(a, 5, 4);
        var c = "[1,2,3,4,null,null,null,5,5]";
        Assert.AreEqual(c, b.ToString());

    }

    [TestMethod]
    public void TestMethod4()
    {
    }

}