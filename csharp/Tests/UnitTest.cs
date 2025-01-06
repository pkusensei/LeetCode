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
        var a = TreeNode.MakeInt([1, 2, 3, 4, 5, 6]);
        Assert.AreEqual(110, sol.MaxProduct(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1,null,2,3,4,null,null,5,6]);
        Assert.AreEqual(90, sol.MaxProduct(a));
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