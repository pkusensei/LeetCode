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
        var a = TreeNode.Make([1, 2]);
        var b = sol.PrintTree(a);
        var c = """[["","1",""],["2","",""]]""";
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([1, 2, 3, null, 4]);
        var b = sol.PrintTree(a);
        var c = """[["","","","1","","",""],["","2","","","","3",""],["","","4","","","",""]]""";
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