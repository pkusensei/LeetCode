using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("110", "[1,1,3]")]
    [DataRow("001011", "[11,8,5,4,3,4]")]
    public void TestMethod1(string boxes, string exp)
    {
        Assert.AreEqual(exp, sol.MinOperations(boxes).Print());
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