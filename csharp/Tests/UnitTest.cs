using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, "[3,1,2,3,2]")]
    [DataRow(5, "[5,3,1,4,3,5,2,4,2]")]
    public void TestMethod1(int n, string exp)
    {
        Assert.AreEqual(exp, sol.ConstructDistancedSequence(n).Print());
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