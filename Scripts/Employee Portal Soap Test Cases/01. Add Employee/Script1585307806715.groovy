import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import groovy.json.JsonSlurper as JsonSlurper

Response = WS.sendRequest(findTestObject('EmployeePortalSoapRequest/1. Add Employee'))

WS.verifyResponseStatusCode(Response, 200)

assertThat(Response.getStatusCode()).isEqualTo(200)

WS.verifyElementText(Response, 'addEmployeeResponse.return', 'true')

//assertThat(Response.getResponseText()).contains('true')

assertThat(Response.getHeaderFields().get('Content-Type').toString()).isEqualTo('[text/xml; charset=utf-8]')

assertThat(Response.getHeaderFields().get('Connection').toString()).isEqualTo('[Keep-Alive]')

WS.verifyElementPropertyValue(Response, 'addEmployeeResponse.return', 'true')

