using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9 }, 13)]
    public void TestMethod1(int[] exp, int n)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.LexicalOrder(n)));
    }

    [TestMethod]
    public void TestMethod2()
    {
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