<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>1. Add Employee</name>
   <tag></tag>
   <elementGuidId>26b9b511-9388-4bab-bd64-98d08531e38b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;addEmployee xmlns=&quot;http://sample.com/reservation/guest/types&quot;>
            &lt;name>Ajay123&lt;/name>
            &lt;id>569&lt;/id>
            &lt;Department>GESS&lt;/Department>
            &lt;age>78&lt;/age>
        &lt;/addEmployee>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addEmployee</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementText(response, 'addEmployeeResponse.return', &quot;true&quot;)

assertThat(response.getResponseText()).contains('true')


assertThat(response.getHeaderFields().get('Content-Type').toString()).isEqualTo('[text/xml; charset=utf-8]')

assertThat(response.getHeaderFields().containsKey('Content-Type')).isTrue()




</verificationScript>
   <wsdlAddress>http://216.10.245.166:8080/axis2/services/EmployeeManagementService?wsdl</wsdlAddress>
</WebServiceRequestEntity>
