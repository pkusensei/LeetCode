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
        int[][] des = [[20, 15, 1], [20, 17, 0], [50, 20, 1], [50, 80, 0], [80, 19, 1]];
        Assert.AreEqual("[50,20,80,15,17,19]", sol.CreateBinaryTree(des).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] des = [[1, 2, 1], [2, 3, 0], [3, 4, 1]];
        Assert.AreEqual("[1,2,null,null,3,4]", sol.CreateBinaryTree(des).ToString());
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