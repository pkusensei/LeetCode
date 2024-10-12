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
        var a = TreeNode.Make([4, 2, 7, 1, 3]);
        var b = sol.InsertIntoBST(a, 5);
        var c = "[4,2,7,1,3,5]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([40, 20, 60, 10, 30, 50, 70]);
        var b = sol.InsertIntoBST(a, 25);
        var c = "[40,20,60,10,30,50,70,null,null,25]";
        Assert.AreEqual(c, b.ToString());
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